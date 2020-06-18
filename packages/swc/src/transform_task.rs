use std::str;

use napi::{Buffer, Env, Error, JsString, Result, Status, Task, Value};

use std::sync::Arc;
use swc::{
  common::{self, errors::Handler, FileName, FilePathMapping, SourceMap},
  config::{Config, JscConfig, Options},
  ecmascript::parser::{JscTarget, Syntax, TsConfig},
  Compiler,
};

pub struct TransformTask {
  source: Value<Buffer>,
}

impl TransformTask {
  pub fn new(source: Value<Buffer>) -> Self {
    Self { source }
  }

  #[inline]
  pub fn perform(source: Value<Buffer>) -> Result<String> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Handler::with_tty_emitter(
      common::errors::ColorConfig::Always,
      true,
      false,
      Some(cm.clone()),
    );
    let c = Compiler::new(cm.clone(), Arc::new(handler));
    let fm = c.cm.new_source_file(
      FileName::Anon,
      str::from_utf8(&source)
        .map_err(|e| Error {
          status: Status::StringExpected,
          reason: Some(format!("{:?}", e)),
        })?
        .to_owned(),
    );
    let program = c.run(|| {
      c.parse_js(
        fm,
        JscTarget::Es2018,
        Syntax::Typescript(TsConfig::default()),
        true,
        true,
      )
      .map_err(|e| Error {
        status: Status::GenericFailure,
        reason: Some(format!("{:?}", e)),
      })
    })?;
    let result = c.run(|| {
      serde_json::to_string(&program).map_err(|e| Error {
        status: Status::GenericFailure,
        reason: Some(format!("{:?}", e)),
      })
    })?;
    Ok(result)
  }
}

impl Task for TransformTask {
  type Output = String;
  type JsValue = JsString;

  fn compute(&self) -> Result<Self::Output> {
    TransformTask::perform(self.source)
  }

  fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Value<Self::JsValue>> {
    env.create_string(output.as_str())
  }
}
