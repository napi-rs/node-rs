#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::str;

use deno_lint::ast_parser::get_default_ts_config;
use deno_lint::diagnostic::LintDiagnostic;
use deno_lint::linter::LinterBuilder;
use deno_lint::rules::{get_all_rules, get_recommended_rules};
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use napi::{CallContext, Error, JsBoolean, JsBuffer, JsObject, JsString, Result, Status};
use swc_ecmascript::parser::Syntax;
use swc_ecmascript::parser::TsConfig;
use termcolor::Color::{Ansi256, Red};
use termcolor::{Ansi, ColorSpec, WriteColor};

#[cfg(windows)]
use termcolor::{BufferWriter, ColorChoice};

#[cfg(all(unix, not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

fn bold(s: String) -> impl fmt::Display {
  let mut style_spec = ColorSpec::new();
  style_spec.set_bold(true);
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

pub fn format_diagnostic(diagnostic: &LintDiagnostic, source: &str) -> String {
  let pretty_error = format!(
    "({}) {}",
    gray(diagnostic.code.to_string()),
    diagnostic.message
  );

  let file_name = &diagnostic.filename;
  let location =
    if file_name.contains('/') || file_name.contains('\\') || file_name.starts_with("./") {
      file_name.to_string()
    } else {
      format!("./{}", file_name)
    };

  let line_str_len = diagnostic.range.end.line.to_string().len();
  let pretty_location = cyan(format!(
    "{}--> {}:{}:{}",
    " ".repeat(line_str_len),
    location,
    diagnostic.range.start.line,
    diagnostic.range.start.col
  ))
  .to_string();

  let dummy = format!("{} |", " ".repeat(line_str_len));

  if diagnostic.range.start.line == diagnostic.range.end.line {
    let snippet_length = diagnostic.range.end.col - diagnostic.range.start.col;
    let source_lines: Vec<&str> = source.split('\n').collect();
    let line = source_lines[diagnostic.range.start.line - 1];
    let pretty_line_src = format!("{} | {}", diagnostic.range.start.line, line);
    let red_glyphs = format!(
      "{} | {}{}",
      " ".repeat(line_str_len),
      " ".repeat(diagnostic.range.start.col),
      red("^".repeat(snippet_length))
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
  } else {
    let mut lines = vec![pretty_error, pretty_location, dummy.clone()];
    let source_lines: Vec<&str> = source.split('\n').collect();

    for i in diagnostic.range.start.line..(diagnostic.range.end.line + 1) {
      let line = source_lines[i - 1];
      let is_first = i == diagnostic.range.start.line;
      let is_last = i == diagnostic.range.end.line;

      if is_first {
        let (rest, snippet) = line.split_at(diagnostic.range.start.col);
        lines.push(format!("{} |   {}{}", i, rest, bold(snippet.to_string())));
      } else if is_last {
        let (snippet, rest) = line.split_at(diagnostic.range.end.col);
        lines.push(format!(
          "{} | {} {}{}",
          i,
          red("|".to_string()),
          bold(snippet.to_string()),
          rest
        ));
      } else {
        lines.push(format!(
          "{} | {} {}",
          i,
          red("|".to_string()),
          bold(line.to_string())
        ));
      }

      // If this is the first line, render the ∨ symbols
      if is_first {
        lines.push(format!(
          "{} |  {}{}",
          " ".repeat(line_str_len),
          red("_".repeat(diagnostic.range.start.col + 1)),
          red("^".to_string())
        ));
      }

      // If this is the last line, render the ∨ symbols
      if is_last {
        lines.push(format!(
          "{} | {}{}{}",
          " ".repeat(line_str_len),
          red("|".to_string()),
          red("_".repeat(diagnostic.range.end.col)),
          red("^".to_string())
        ));
      }
    }

    lines.push(dummy);

    lines.join("\n")
  }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("lint", lint)?;
  exports.create_named_method("denolint", lint_command)?;

  Ok(())
}

#[js_function(3)]
fn lint(ctx: CallContext) -> Result<JsObject> {
  let file_name = ctx.get::<JsString>(0)?.into_utf8()?;
  let source_code = ctx.get::<JsBuffer>(1)?.into_value()?;
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

  let (_, file_diagnostics) = linter
    .lint(file_name_ref.to_owned(), source_string.to_owned())
    .map_err(|e| Error {
      status: Status::GenericFailure,
      reason: format!("Lint failed: {}, at: {}", e, file_name_ref),
    })?;

  let mut result = ctx.env.create_array_with_length(file_diagnostics.len())?;

  for (index, diagnostic) in file_diagnostics.iter().enumerate() {
    result.set_element(
      index as _,
      ctx
        .env
        .create_string(format_diagnostic(diagnostic, source_string).as_str())?,
    )?;
  }

  Ok(result)
}

#[js_function(2)]
fn lint_command(ctx: CallContext) -> Result<JsBoolean> {
  let __dirname = ctx.get::<JsString>(0)?.into_utf8()?;
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

  let mut denolint_ignore_file = cwd.clone();

  denolint_ignore_file.push(".denolintignore");

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

  let ignore_file_path = match fs::File::open(&denolint_ignore_file) {
    Ok(_) => denolint_ignore_file.as_path().to_str().ok_or_else(|| {
      Error::from_reason(format!(
        "Convert path to string failed: {:?}",
        &denolint_ignore_file
      ))
    })?,
    Err(_) => match fs::File::open(&eslint_ignore_file) {
      Ok(_) => eslint_ignore_file.as_path().to_str().ok_or_else(|| {
        Error::from_reason(format!(
          "Convert path to string failed: {:?}",
          &eslint_ignore_file
        ))
      })?,
      Err(_) => __dirname.as_str()?,
    },
  };

  for result in WalkBuilder::new(cwd)
    .add_custom_ignore_filename(ignore_file_path)
    .types(types)
    .follow_links(true)
    .build()
  {
    if let Ok(entry) = result {
      let p = entry.path();
      if !p.is_dir() {
        let file_content = fs::read_to_string(&p)
          .map_err(|e| Error::from_reason(format!("Read file {:?} failed: {}", p, e)))?;

        let ts_config = TsConfig {
          dynamic_import: true,
          decorators: true,
          tsx: p.ends_with(".tsx"),
          ..Default::default()
        };
        let syntax = Syntax::Typescript(ts_config);
        let mut linter = LinterBuilder::default()
          .rules(if enable_all_rules {
            get_all_rules()
          } else {
            get_recommended_rules()
          })
          .syntax(syntax)
          .build();
        let (_, file_diagnostics) = linter
          .lint(
            (&p.to_str())
              .ok_or(Error::from_reason(format!(
                "Convert path to string failed: {:?}",
                &p
              )))?
              .to_owned(),
            file_content.clone(),
          )
          .map_err(|e| Error {
            status: Status::GenericFailure,
            reason: format!("Lint failed: {}, at: {:?}", e, &p),
          })?;
        for diagnostic in file_diagnostics {
          has_error = true;
          println!("{}", format_diagnostic(&diagnostic, file_content.as_str()));
        }
      }
    };
  }

  ctx.env.get_boolean(has_error)
}
