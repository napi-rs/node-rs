#![deny(clippy::all)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use std::env;
use std::fs;
use std::path;
use std::path::Path;
use std::str;

use deno_ast::MediaType;
use deno_lint::linter::LinterBuilder;
use deno_lint::rules::{get_all_rules, get_recommended_rules};
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use napi::bindgen_prelude::*;
use napi_derive::*;

mod config;
mod diagnostics;

#[inline(always)]
fn get_media_type(p: &Path) -> MediaType {
  match p.extension().and_then(|e| e.to_str()) {
    Some("tsx") => MediaType::Tsx,
    Some("jsx") => MediaType::Jsx,
    Some("js") | Some("mjs") => MediaType::JavaScript,
    Some("ts") => MediaType::TypeScript,
    _ => MediaType::Tsx,
  }
}

#[napi]
fn lint(
  file_name: String,
  source_code: Either<String, Buffer>,
  all_rules: Option<bool>,
) -> Result<Vec<String>> {
  let all_rules = all_rules.unwrap_or(false);
  let linter = LinterBuilder::default()
    .rules(if all_rules {
      get_all_rules()
    } else {
      get_recommended_rules()
    })
    .media_type(get_media_type(Path::new(file_name.as_str())))
    .ignore_diagnostic_directive("eslint-disable-next-line")
    .build();

  let source_string = match &source_code {
    Either::A(s) => s,
    Either::B(b) => str::from_utf8(b.as_ref()).map_err(|e| {
      Error::new(
        Status::StringExpected,
        format!("Input source is not valid utf8 string {}", e),
      )
    })?,
  };

  let (s, file_diagnostics) = linter
    .lint(file_name.clone(), source_string.to_owned())
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Lint failed: {}, at: {}", e, file_name),
      )
    })?;

  Ok(
    diagnostics::display_diagnostics(&file_diagnostics, s.source(), false)
      .into_iter()
      .map(|d| format!("{}", d))
      .collect(),
  )
}

#[napi]
fn denolint(__dirname: String, config_path: String) -> Result<bool> {
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
    (cfg.get_rules(), cfg.files.exclude)
  } else {
    (get_recommended_rules(), vec![])
  };

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
      Err(_) => __dirname.as_str(),
    },
  };
  let mut dir_walker = WalkBuilder::new(cwd);
  dir_walker
    .add_custom_ignore_filename(ignore_file_path)
    .types(types)
    .follow_links(true);
  for i in cfg_ignore_files {
    dir_walker.add_ignore(i);
  }
  for entry in dir_walker.build().filter_map(|v| v.ok()) {
    let p = entry.path();
    if !p.is_dir() {
      let file_content = fs::read_to_string(&p)
        .map_err(|e| Error::from_reason(format!("Read file {:?} failed: {}", p, e)))?;

      let linter = LinterBuilder::default()
        .rules(rules.clone())
        .media_type(get_media_type(p))
        .ignore_file_directive("eslint-disable")
        .ignore_diagnostic_directive("eslint-disable-next-line")
        .build();
      let (s, file_diagnostics) = linter
        .lint(
          p.to_str()
            .ok_or_else(|| Error::from_reason(format!("Convert path to string failed: {:?}", &p)))?
            .to_owned(),
          file_content.clone(),
        )
        .map_err(|e| {
          Error::new(
            Status::GenericFailure,
            format!("Lint failed: {}, at: {:?}", e, &p),
          )
        })?;
      has_error = has_error || !file_diagnostics.is_empty();
      diagnostics::display_diagnostics(&file_diagnostics, s.source(), true);
    }
  }

  Ok(has_error)
}
