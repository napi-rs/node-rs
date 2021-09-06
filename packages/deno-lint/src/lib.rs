#![deny(clippy::all)]
#![allow(clippy::nonstandard_macro_braces)]

#[macro_use]
extern crate napi_derive;

use std::env;
use std::fs;
use std::path;
use std::str;
use std::sync::Arc;

use deno_ast::swc::parser::{Syntax, TsConfig};
use deno_lint::ast_parser::get_default_ts_config;
use deno_lint::linter::LinterBuilder;
use deno_lint::rules::{get_all_rules, get_recommended_rules};
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use napi::{CallContext, Error, JsBoolean, JsBuffer, JsObject, JsString, Result, Status};

mod config;
mod diagnostics;

#[cfg(all(
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  env_logger::init();

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
    .ignore_diagnostic_directive("eslint-disable-next-line")
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

  let d = diagnostics::display_diagnostics(&file_diagnostics, s.source(), false);
  for (index, diagnostic) in d.iter().enumerate() {
    result.set_element(
      index as _,
      ctx.env.create_string_from_std(format!("{}", diagnostic))?,
    )?;
  }

  Ok(result)
}

#[js_function(2)]
fn lint_command(ctx: CallContext) -> Result<JsBoolean> {
  let __dirname = ctx.get::<JsString>(0)?.into_utf8()?;
  let config_path_js = ctx.get::<JsString>(1)?.into_utf8()?;
  let config_path = config_path_js.as_str()?;
  let mut has_error = false;
  let cwd = env::current_dir().map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Get current_dir failed {}", e),
    )
  })?;
  let config_existed = fs::metadata(&config_path)
    .map(|m| m.is_file())
    .unwrap_or(false);

  let (rules, cfg_ignore_files) = if config_existed {
    let cfg = config::load_from_json(path::Path::new(&config_path))?;
    (cfg.get_rules(), cfg.ignore)
  } else {
    (get_recommended_rules(), None)
  };

  let mut eslint_ignore_file = cwd.clone();

  eslint_ignore_file.push(".eslintignore");

  let mut denolint_ignore_file = cwd.clone();

  denolint_ignore_file.push(".denolintignore");

  if let Some(ignore_files) = cfg_ignore_files {
    for i in ignore_files {
      denolint_ignore_file.push(i);
    }
  }

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
        .rules(Arc::clone(&rules))
        .syntax(syntax)
        .ignore_file_directive("eslint-disable")
        .ignore_diagnostic_directive("eslint-disable-next-line")
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
      has_error = !file_diagnostics.is_empty();
      diagnostics::display_diagnostics(&file_diagnostics, s.source(), true);
    }
  }

  ctx.env.get_boolean(has_error)
}
