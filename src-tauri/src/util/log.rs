use chrono::Local;
use colored::Colorize;
use std::fmt::Display;

pub enum LogKind {
  Info,
  Success,
  Warn,
  Error,
}

pub fn log(s: impl AsRef<str> + Display, kind: Option<LogKind>) {
  let status = match kind {
    Some(LogKind::Info) => "INFO".blue(),
    Some(LogKind::Success) => "SUCCESS".green(),
    Some(LogKind::Warn) => "WARN".yellow(),
    Some(LogKind::Error) => "ERROR".red(),
    None => "INFO".blue(),
  };

  println!(
    "[{}] [{}] {}",
    Local::now().format("%Y-%m-%d %H:%M:%S"),
    status,
    s
  );
}

#[macro_export]
macro_rules! log {
  ($($arg:tt)*) => {
    $crate::util::log::log(format!($($arg)*), Some($crate::util::log::LogKind::Info))
  };
}

#[macro_export]
macro_rules! success {
  ($($arg:tt)*) => {
    $crate::util::log::log(format!($($arg)*), Some($crate::util::log::LogKind::Success))
  };
}

#[macro_export]
macro_rules! warn {
  ($($arg:tt)*) => {
    $crate::util::log::log(format!($($arg)*), Some($crate::util::log::LogKind::Warn))
  };
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    $crate::util::log::log(format!($($arg)*), Some($crate::util::log::LogKind::Error))
  };
}
