#![deny(clippy::all)]
#![allow(clippy::nonstandard_macro_braces)]

#[macro_use]
extern crate napi_derive;

use std::env;
use std::fs;
use std::str;

use annotate_snippets::{display_list, snippet};
use ast_view::{SourceFile, SourceFileTextInfo};
use deno_lint::ast_parser::get_default_ts_config;
use deno_lint::diagnostic::{LintDiagnostic, Range};
use deno_lint::linter::LinterBuilder;
use deno_lint::rules::{get_all_rules, get_recommended_rules};
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use napi::{CallContext, Error, JsBoolean, JsBuffer, JsObject, JsString, Result, Status};
use swc_ecmascript::parser::Syntax;
use swc_ecmascript::parser::TsConfig;

#[cfg(all(
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Return slice of source code covered by diagnostic
// and adjusted range of diagnostic (ie. original range - start line
// of sliced source code).
fn get_slice_source_and_range<'a>(
  source_file: &'a SourceFileTextInfo,
  range: &Range,
) -> (&'a str, (usize, usize)) {
  let first_line_start = source_file.line_start(range.start.line_index).0 as usize;
  let last_line_end = source_file.line_end(range.end.line_index).0 as usize;
  let adjusted_start = range.start.byte_pos - first_line_start;
  let adjusted_end = range.end.byte_pos - first_line_start;
  let adjusted_range = (adjusted_start, adjusted_end);
  let slice_str = &source_file.text()[first_line_start..last_line_end];
  (slice_str, adjusted_range)
}

fn format_diagnostic(diagnostic: &LintDiagnostic, source_file: &SourceFileTextInfo) -> String {
  let (slice_source, range) = get_slice_source_and_range(source_file, &diagnostic.range);
  let footer = if let Some(hint) = &diagnostic.hint {
    vec![snippet::Annotation {
      label: Some(hint),
      id: None,
      annotation_type: snippet::AnnotationType::Help,
    }]
  } else {
    vec![]
  };

  let snippet = snippet::Snippet {
    title: Some(snippet::Annotation {
      label: Some(&diagnostic.message),
      id: Some(&diagnostic.code),
      annotation_type: snippet::AnnotationType::Error,
    }),
    footer,
    slices: vec![snippet::Slice {
      source: slice_source,
      line_start: diagnostic.range.start.line_index + 1, // make 1-indexed
      origin: Some(&diagnostic.filename),
      fold: false,
      annotations: vec![snippet::SourceAnnotation {
        range,
        label: "",
        annotation_type: snippet::AnnotationType::Error,
      }],
    }],
    opt: display_list::FormatOptions {
      color: true,
      anonymized_line_numbers: false,
      margin: None,
    },
  };
  let display_list = display_list::DisplayList::from(snippet);
  format!("{}", display_list)
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
  let linter = LinterBuilder::default()
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

  let (s, file_diagnostics) = linter
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
        .create_string(format_diagnostic(&diagnostic, &s).as_str())?,
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

  for entry in WalkBuilder::new(cwd)
    .add_custom_ignore_filename(ignore_file_path)
    .types(types)
    .follow_links(true)
    .build()
    .filter_map(|v| v.ok())
  {
    let p = entry.path();
    if !p.is_dir() {
      let file_content = fs::read_to_string(&p)
        .map_err(|e| Error::from_reason(format!("Read file {:?} failed: {}", p, e)))?;

      let ts_config = TsConfig {
        dynamic_import: true,
        decorators: true,
        tsx: p
          .extension()
          .and_then(|ext| ext.to_str())
          .map(|ext| ext == "tsx")
          .unwrap_or(false),
        ..Default::default()
      };
      let syntax = Syntax::Typescript(ts_config);
      let linter = LinterBuilder::default()
        .rules(if enable_all_rules {
          get_all_rules()
        } else {
          get_recommended_rules()
        })
        .syntax(syntax)
        .build();
      let (s, file_diagnostics) = linter
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
        println!("{}", format_diagnostic(&diagnostic, &s));
      }
    }
  }

  ctx.env.get_boolean(has_error)
}
