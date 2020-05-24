use std::str;

use napi::{Buffer, Env, Error, JsString, Result, Status, Task, Value};

use std::sync::Arc;
use swc::{
  common::{self, errors::Handler, FileName, FilePathMapping, SourceMap},
  config::{Config, InputSourceMap, JscTarget, Options},
  ecmascript::parser::Syntax,
  Compiler,
};

pub struct UglifyTask {
  source: Value<Buffer>,
}

impl UglifyTask {
  pub fn new(source: Value<Buffer>) -> UglifyTask {
    Self { source }
  }

  #[inline]
  pub fn uglify(source: Value<Buffer>) -> Result<String> {
    let source_code =
      str::from_utf8(&source).map_err(|_| Error::from_status(Status::StringExpected))?;
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Handler::with_tty_emitter(
      common::errors::ColorConfig::Always,
      true,
      false,
      Some(cm.clone()),
    );
    let c = Compiler::new(cm.clone(), handler);
    let fm = c.cm.new_source_file(FileName::Anon, source_code.to_owned());

    let program = c
      .parse_js(
        fm,
        JscTarget::Es5,
        Syntax::default(),
        false,
        false,
        &InputSourceMap::Bool(false),
      )
      .map(|v| v.0)
      .map_err(|e| Error {
        status: Status::GenericFailure,
        reason: Some(format!("{:?}", e)),
      })?;
    let result = c
      .process_js(
        program,
        None,
        &Options {
          config: Some(Config {
            env: None,
            test: None,
            exclude: None,
            minify: Some(true),
            ..Config::default()
          }),
          is_module: false,
          swcrc: false,
          ..Options::default()
        },
      )
      .map_err(|e| Error {
        status: Status::GenericFailure,
        reason: Some(format!("{:?}", e)),
      })?;
    Ok(result.code)
  }
}

impl Task for UglifyTask {
  type Output = String;
  type JsValue = JsString;

  fn compute(&self) -> Result<Self::Output> {
    UglifyTask::uglify(self.source)
  }

  fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Value<Self::JsValue>> {
    env.create_string(output.as_str())
  }
}
