#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::str;

use deno_lint::diagnostic::LintDiagnostic;
use deno_lint::linter::LinterBuilder;
use deno_lint::rules::{get_all_rules, get_recommended_rules};
use deno_lint::swc_util::get_default_ts_config;
use ignore::overrides::OverrideBuilder;
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use napi::{CallContext, Error, JsBoolean, JsBuffer, JsObject, JsString, Module, Result, Status};
use termcolor::Color::{Ansi256, Red};
use termcolor::{Ansi, ColorSpec, WriteColor};

#[cfg(windows)]
use termcolor::{BufferWriter, ColorChoice};

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

register_module!(denolint, init);

#[allow(unused)]
#[cfg(windows)]
fn enable_ansi() {
  BufferWriter::stdout(ColorChoice::AlwaysAnsi);
}

fn gray(s: String) -> impl fmt::Display {
  let mut style_spec = ColorSpec::new();
  style_spec.set_fg(Some(Ansi256(8)));
  style(&s, style_spec)
}

fn red(s: String) -> impl fmt::Display {
  let mut style_spec = ColorSpec::new();
  style_spec.set_fg(Some(Red));
  style(&s, style_spec)
}

fn cyan(s: String) -> impl fmt::Display {
  let mut style_spec = ColorSpec::new();
  style_spec.set_fg(Some(Ansi256(14)));
  style(&s, style_spec)
}

fn style(s: &str, colorspec: ColorSpec) -> impl fmt::Display {
  let mut v = Vec::new();
  let mut ansi_writer = Ansi::new(&mut v);
  ansi_writer.set_color(&colorspec).unwrap();
  ansi_writer.write_all(s.as_bytes()).unwrap();
  ansi_writer.reset().unwrap();
  String::from_utf8_lossy(&v).into_owned()
}

pub fn format_diagnostic(diagnostic: &LintDiagnostic) -> String {
  let pretty_error = format!(
    "({}) {}",
    gray(diagnostic.code.to_string()),
    diagnostic.message
  );

  let file_name = &diagnostic.location.filename;
  let location =
    if file_name.contains('/') || file_name.contains('\\') || file_name.starts_with("./") {
      file_name.to_string()
    } else {
      format!("./{}", file_name)
    };

  let line_str_len = diagnostic.location.line.to_string().len();
  let pretty_location = cyan(format!(
    "{}--> {}:{}:{}",
    " ".repeat(line_str_len),
    location,
    diagnostic.location.line,
    diagnostic.location.col
  ))
  .to_string();

  let dummy = format!("{} |", " ".repeat(line_str_len));
  let pretty_line_src = format!("{} | {}", diagnostic.location.line, diagnostic.line_src);
  let red_glyphs = format!(
    "{} | {}{}",
    " ".repeat(line_str_len),
    " ".repeat(diagnostic.location.col),
    red("^".repeat(diagnostic.snippet_length))
  );

  let lines = vec![
    pretty_error,
    pretty_location,
    dummy.clone(),
    pretty_line_src,
    red_glyphs,
    dummy,
  ];

  lines.join("\n")
}

fn init(js_module: &mut Module) -> Result<()> {
  js_module.create_named_method("lint", lint)?;
  js_module.create_named_method("denolint", lint_command)?;

  Ok(())
}

#[js_function(3)]
fn lint(ctx: CallContext) -> Result<JsObject> {
  let file_name = ctx.get::<JsString>(0)?;
  let source_code = ctx.get::<JsBuffer>(1)?;
  let all_rules = ctx.get::<JsBoolean>(2)?;
  let mut linter = LinterBuilder::default()
    .rules(if all_rules.get_value()? {
      get_all_rules()
    } else {
      get_recommended_rules()
    })
    .syntax(get_default_ts_config())
    .build();

  let source_string = str::from_utf8(&source_code).map_err(|e| Error {
    status: Status::StringExpected,
    reason: format!("Input source is not valid utf8 string {}", e),
  })?;

  let file_name_ref = file_name.as_str()?;

  let file_diagnostics = linter
    .lint(file_name_ref.to_owned(), source_string.to_owned())
    .map_err(|e| Error {
      status: Status::GenericFailure,
      reason: format!("Lint failed: {}, at: {}", e, file_name_ref),
    })?;

  let mut result = ctx.env.create_array_with_length(file_diagnostics.len())?;

  for (index, diagnostic) in file_diagnostics.iter().enumerate() {
    result.set_number_indexed_property(
      ctx.env.create_int32(index as i32)?,
      ctx
        .env
        .create_string(format_diagnostic(diagnostic).as_str())?,
    )?;
  }

  Ok(result)
}

#[js_function(2)]
fn lint_command(ctx: CallContext) -> Result<JsBoolean> {
  let __dirname = ctx.get::<JsString>(0)?;
  let enable_all_rules = ctx.get::<JsBoolean>(1)?.get_value()?;
  let mut has_error = false;
  let cwd = env::current_dir().map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Get current_dir failed {}", e),
    )
  })?;

  let mut eslint_ignore_file = cwd.clone();

  eslint_ignore_file.push(".eslintignore");

  let mut type_builder = TypesBuilder::new();

  type_builder
    .add("typescript", "*.ts")
    .map_err(|e| Error::from_reason(format!("{}", e)))?;
  type_builder
    .add("typescript", "*.tsx")
    .map_err(|e| Error::from_reason(format!("{}", e)))?;

  let types = type_builder
    .add_defaults()
    .select("typescript")
    .select("js")
    .build()
    .map_err(|e| Error::from_reason(format!("{}", e)))?;

  let override_ignore = match fs::File::open(&eslint_ignore_file) {
    Ok(_) => OverrideBuilder::new(eslint_ignore_file)
      .build()
      .map_err(|e| {
        Error::from_reason(format!(
          "Create ignore rules from .eslintignore file failed {}",
          e
        ))
      })?,
    Err(_) => OverrideBuilder::new(__dirname.as_str()?)
      .build()
      .map_err(|e| {
        Error::from_reason(format!(
          "Create ignore rules from .defaultignore file failed {}",
          e
        ))
      })?,
  };

  for result in WalkBuilder::new(cwd)
    .overrides(override_ignore)
    .types(types)
    .follow_links(true)
    .build()
  {
    match result {
      Ok(entry) => {
        let p = entry.path();
        if !p.is_dir() {
          let file_content = fs::read_to_string(&p)
            .map_err(|e| Error::from_reason(format!("Read file {:?} failed: {}", p, e)))?;
          let mut linter = LinterBuilder::default()
            .rules(if enable_all_rules {
              get_all_rules()
            } else {
              get_recommended_rules()
            })
            .syntax(get_default_ts_config())
            .build();
          let file_diagnostics = linter
            .lint(
              (&p.to_str())
                .ok_or(Error::from_reason(format!(
                  "Convert path to string failed: {:?}",
                  &p
                )))?
                .to_owned(),
              file_content,
            )
            .map_err(|e| Error {
              status: Status::GenericFailure,
              reason: format!("Lint failed: {}, at: {:?}", e, &p),
            })?;
          for diagnostic in file_diagnostics {
            has_error = true;
            println!("{}", format_diagnostic(&diagnostic));
          }
        }
      }
      Err(_) => {}
    };
  }

  ctx.env.get_boolean(has_error)
}
