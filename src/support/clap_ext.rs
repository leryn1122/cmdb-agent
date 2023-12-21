use std::ffi::OsStr;
use std::str::FromStr;

use clap::builder::TypedValueParser;
use clap::error::ErrorKind;
use clap::Arg;
use clap::Command;
use clap::Error;

#[derive(Clone)]
pub struct LogLevelValueParser;

impl TypedValueParser for LogLevelValueParser {
  type Value = log::Level;

  fn parse_ref(
    &self,
    _cmd: &Command,
    _arg: Option<&Arg>,
    value: &OsStr,
  ) -> Result<Self::Value, Error> {
    let level = value.to_str().ok_or(Error::new(ErrorKind::InvalidUtf8))?;
    log::Level::from_str(level).map_err(|e| Error::raw(ErrorKind::InvalidValue, e.to_string()))
  }
}
