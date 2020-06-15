#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use deno_lint::diagnostic::LintDiagnostic;
use deno_lint::linter::LinterBuilder;
use napi::{CallContext, Error, JsBuffer, JsObject, JsString, Module, Result, Status};
use std::fmt;
use std::io::Write;
use std::str;
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

  Ok(())
}

#[js_function(2)]
fn lint(ctx: CallContext) -> Result<JsObject> {
  let file_name = ctx.get::<JsString>(0)?;
  let source_code = ctx.get::<JsBuffer>(1)?;
  let mut linter = LinterBuilder::default().build();

  let source_string = str::from_utf8(&source_code).map_err(|e| Error {
    status: Status::StringExpected,
    reason: format!("Input source is not valid utf8 string {}", e),
  })?;

  let file_diagnostics = linter
    .lint(file_name.as_str()?.to_owned(), source_string.to_owned())
    .map_err(|e| Error {
      status: Status::GenericFailure,
      reason: format!("Lint failed: {}", e),
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
