use std::fmt::{Debug, Display, Formatter};
use serde::ser;

#[derive(Debug)]
pub struct Error{
  pub column: usize,
  pub line: usize,
  pub code: ErrorCode,
}

impl Error{
  pub fn code(code: ErrorCode) -> Self{
    Self{
      column: 0,
      line: 0,
      code,
    }
  }
}

impl Display for Error{
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result{
    write!(f, "{:?}", self.code)
  }
}
impl std::error::Error for Error{

}

impl ser::Error for Error{
  fn custom<T: Display>(msg: T) -> Self{
    Error{
      column: 0,
      line: 0,
      code: ErrorCode::Message(msg.to_string()),
    }
  }
}

#[derive(Debug)]
pub enum ErrorCode{
  KeyMustBeAString,
  FloatKeyMustBeFinite,
  IdMapKeyMustBeAnInteger,
  Message(String),
}