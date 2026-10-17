extern crate dprint_development;
extern crate dprint_plugin_cedar;

use std::path::PathBuf;
use std::sync::Arc;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_cedar::configuration::Configuration;
use dprint_plugin_cedar::configuration::resolve_config;
use dprint_plugin_cedar::*;

#[test]
fn test_specs() {
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.cedar",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    {
      let global_config = global_config.clone();
      Arc::new(move |file_path, file_text, _range, spec_config| {
        let spec_config: ConfigKeyMap = serde_json::from_value(spec_config.clone().into()).unwrap();
        let config_result = resolve_config(spec_config, &global_config);
        ensure_no_diagnostics(&config_result.diagnostics);

        format_text(file_path, file_text, &config_result.config).map_err(|err| err.into())
      })
    },
    Arc::new(move |_file_path, _file_text, _spec_config| panic!("Plugin does not support dprint-core tracing.")),
  )
}

#[test]
fn should_fail_on_parse_error() {
  let err = format_text(
    &PathBuf::from("./file.cedar"),
    "permit(principal, action, resource)\nwhen { 1 + };\n",
    &default_config(),
  )
  .unwrap_err();
  let message = err.to_string();
  assert!(
    message.starts_with("cannot parse input policies: unexpected token `}`"),
    "{message}"
  );
  assert!(message.contains("at line 2, column 12"), "{message}");
}

#[test]
fn should_use_crlf_when_configured() {
  let mut config = default_config();
  config.new_line_kind = NewLineKind::CarriageReturnLineFeed;
  let result = format_text(
    &PathBuf::from("./file.cedar"),
    "permit(principal,action,resource);\n",
    &config,
  )
  .unwrap();
  assert_eq!(result.unwrap(), "permit (principal, action, resource);\r\n");
}

fn default_config() -> Configuration {
  resolve_config(Default::default(), &GlobalConfiguration::default()).config
}
