use std::fmt::{Debug, Display, Formatter};
use serde::{ser, de};

#[derive(Debug)]
pub struct Error{
  pub column: usize,
  pub line: usize,
  pub code: ErrorCode,
}

impl Error{
  #[cold]
  pub fn code(code: ErrorCode) -> Self{
    Error{
      column: 0,
      line: 0,
      code,
    }
  }

  #[cold]
  pub fn syntax(code: ErrorCode, line: usize, column: usize) -> Self{
    Error{column, line, code}
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

impl de::Error for Error{
  fn custom<T>(msg: T) -> Self
  where
    T: Display
  {
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
  EofWhileParsingValue,
  EofWhileParsingString,
  EofWhileParsingObject,
  EofWhileParsingArray,
  EofWhileParsingIdMap,
  ExpectedSomeIdent,
  ExpectedSomeValue,
  InvalidNumber,
  NumberOutOfRange,
  InvalidUnicodeCodePoint,
  ControlCharacterWhileParsingString,
  InvalidEscape,
  LoneLeadingSurrogateInHexEscape,
  UnexpectedEndOfHexEscape,
  RecursionLimitExceeded,
  ExpectedColon,
  TrailingComma,
  TrailingCharacters,
  ExpectedObjectCommaOrEnd,
  ExpectedArrayCommaOrEnd,
  ExpectedNumericKey,
  ExpectedDoubleQuote,
  Message(String),
}