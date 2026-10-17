use std::path::Path;

use cedar_policy_formatter::Config;
use cedar_policy_formatter::policies_str_to_pretty;
use dprint_core::configuration::resolve_new_line_kind;

use crate::configuration::Configuration;

/// Error that occurred while formatting Cedar text.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
  /// The text could not be parsed or formatted by cedar-policy-formatter.
  #[error("{0}")]
  Cedar(String),
}

pub fn format_text(_file_path: &Path, input_text: &str, config: &Configuration) -> Result<Option<String>, FormatError> {
  let formatted = policies_str_to_pretty(input_text, &build_config(config))
    .map_err(|err| FormatError::Cedar(build_error_message(&err, input_text)))?;

  // cedar-policy-formatter always outputs \n
  let formatted = if resolve_new_line_kind(input_text, config.new_line_kind) == "\n" {
    formatted
  } else {
    formatted.replace('\n', "\r\n")
  };

  if formatted == input_text {
    Ok(None)
  } else {
    Ok(Some(formatted))
  }
}

fn build_config(config: &Configuration) -> Config {
  Config {
    line_width: config.line_width as usize,
    indent_width: config.indent_width as isize,
  }
}

/// Builds a message with the whole error chain and the location of each label,
/// since the error's `Display` implementation only shows the outermost message.
fn build_error_message(err: &miette::Report, text: &str) -> String {
  let mut message = format!("{err:#}");
  for label in err.labels().into_iter().flatten() {
    let (line, column) = line_and_column(text, label.offset());
    message.push_str(&format!("\n  at line {line}, column {column}"));
    if let Some(label_text) = label.label() {
      message.push_str(": ");
      message.push_str(label_text);
    }
  }
  message
}

/// Gets the one-based line and column of a byte offset in the text.
fn line_and_column(text: &str, offset: usize) -> (usize, usize) {
  let mut offset = offset.min(text.len());
  while !text.is_char_boundary(offset) {
    offset -= 1;
  }
  let text_before = &text[..offset];
  let line_start = text_before.rfind('\n').map(|index| index + 1).unwrap_or(0);
  let line = text_before.matches('\n').count() + 1;
  let column = text_before[line_start..].chars().count() + 1;
  (line, column)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn gets_line_and_column() {
    let text = "a\nbc\n\u{e9}d";
    assert_eq!(line_and_column(text, 0), (1, 1));
    assert_eq!(line_and_column(text, 3), (2, 2));
    assert_eq!(line_and_column(text, 5), (3, 1));
    // the middle of a multi-byte character and past the end
    assert_eq!(line_and_column(text, 6), (3, 1));
    assert_eq!(line_and_column(text, 100), (3, 3));
  }
}
