// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.
use deno_lint::rules::{get_all_rules, get_filtered_rules, get_recommended_rules, LintRule};
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RulesConfig {
  pub tags: Vec<String>,
  pub include: Vec<String>,
  pub exclude: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct FilesConfig {
  pub include: Vec<String>,
  pub exclude: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
  pub rules: RulesConfig,
  pub files: FilesConfig,
}

impl Config {
  pub fn get_rules(&self) -> Vec<Arc<dyn LintRule>> {
    get_filtered_rules(
      Some(self.rules.tags.clone()),
      Some(self.rules.exclude.clone()),
      Some(self.rules.include.clone()),
    )
  }
}

pub fn load_from_json(config_path: &Path) -> Result<Config, std::io::Error> {
  let json_str = std::fs::read_to_string(config_path)?;
  let config: Config = serde_json::from_str(&json_str)?;
  Ok(config)
}

pub fn filter_rules(
  all: bool,
  exclude: Option<Vec<String>>,
  include: Option<Vec<String>>,
) -> Vec<Arc<dyn LintRule>> {
  if exclude.is_some() || include.is_some() {
    let tags = if all {
      vec![]
    } else {
      vec!["recommended".to_string()]
    };
    return get_filtered_rules(
      Some(tags),
      Some(exclude.unwrap_or_default()),
      Some(include.unwrap_or_default()),
    );
  }
  if all {
    get_all_rules()
  } else {
    get_recommended_rules()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use deno_lint::rules::get_recommended_rules;
  use std::collections::HashSet;

  macro_rules! svec {
    ($( $elem:literal ),* $(,)?) => {{
      vec![$( $elem.to_string() ),*]
    }}
  }
  macro_rules! set {
    ($( $elem:literal ),* $(,)?) => {{
      vec![$( $elem ),*].into_iter().collect::<HashSet<&'static str>>()
    }}
  }

  fn into_codes(rules: Vec<Arc<dyn LintRule>>) -> HashSet<&'static str> {
    rules.iter().map(|rule| rule.code()).collect()
  }

  #[test]
  fn test_get_rules() {
    let config = Config {
      rules: RulesConfig {
        tags: svec![],
        include: svec![],
        exclude: svec![],
      },
      ..Default::default()
    };
    assert!(config.get_rules().is_empty());

    let config = Config {
      rules: RulesConfig {
        tags: svec!["recommended"],
        include: svec![],
        exclude: svec![],
      },
      ..Default::default()
    };
    let recommended_rules_codes = into_codes(get_recommended_rules());
    assert_eq!(into_codes(config.get_rules()), recommended_rules_codes);

    // even if "recommended" is specified in `tags` and `include` contains a rule
    // code that is in the "recommended" set, we have to make sure that each
    // rule is run just once respectively.
    let config = Config {
      rules: RulesConfig {
        tags: svec!["recommended"],
        include: svec!["no-empty"], // "no-empty" belongs to "recommended"
        exclude: svec![],
      },
      ..Default::default()
    };
    let recommended_rules_codes = into_codes(get_recommended_rules());
    assert_eq!(into_codes(config.get_rules()), recommended_rules_codes);

    // `include` has higher precedence over `exclude`
    let config = Config {
      rules: RulesConfig {
        tags: svec![],
        include: svec!["eqeqeq"],
        exclude: svec!["eqeqeq"],
      },
      ..Default::default()
    };
    assert_eq!(into_codes(config.get_rules()), set!["eqeqeq"]);

    // if unknown rule is specified, just ignore it
    let config = Config {
      rules: RulesConfig {
        tags: svec![],
        include: svec!["this-is-a-totally-unknown-rule"],
        exclude: svec!["this-is-also-another-unknown-rule"],
      },
      ..Default::default()
    };
    assert_eq!(into_codes(config.get_rules()), set![]);
  }
}
