use crate::zil::ast::Node;
use crate::js::contracts::*;

#[macro_export]
macro_rules! wrap {
  ($func:expr, $msg:expr) => {
      match $func {
          Err(e) => return Err(OutputErr::from(HandlerErr::wrap(e, format!("{}\nat {} in {}", $msg, line!(), file!())))),
          Ok(v) => v,
      };
  };
  ($func:expr) => {
    match $func {
        Err(e) => return Err(OutputErr::from(HandlerErr::wrap(e, format!("at {} in {}", line!(), file!())))),
        Ok(v) => v,
    };
  };
}

pub fn escape_text(root: &Node) -> Result<String, OutputErr> {
  if !root.is_text() {
      return Err(OutputErr::from(HandlerErr::origin(format!("bad text to escape: {}", root))));
  }

  let escaped = root.tokens[0].value.replace("\"", "\\\"").replace("\n", "\\n");
  Ok(String::from(escaped))
}

pub fn is_int(root: &Node) -> bool {
  if !root.is_word() {
    return false;
  }

  match root.tokens[0].value.parse::<usize>() { // turbofish!
    Ok(_) => true,
    Err(_) => false
  }
}

pub fn format_keyword(root: &Node) -> Result<String, OutputErr> {
  let (prefix, mut keyword, suffix) = wrap!(crack_keyword(&root), root);

  keyword = match prefix {
    KeywordPrefix::Qfix => format!("pq_{}", &keyword),
    KeywordPrefix::Comma => format!("comma_{}", &keyword),
    KeywordPrefix::Dot => format!("dot_{}", &keyword),
    KeywordPrefix::None => keyword
  };

  keyword = match suffix {
    KeywordSuffix::Question => format!("{}_q", &keyword),
    KeywordSuffix::None => keyword
  };

  Ok(String::from(keyword))
}

pub fn crack_keyword(root: &Node) -> Result<(KeywordPrefix, String, KeywordSuffix), OutputErr> {
  if !root.is_word() && !root.is_text() {
    return Err(OutputErr::from(HandlerErr::origin(format!("bad keyword to crack: {}", root))));
  }

  let prefix: KeywordPrefix;
  let suffix: KeywordSuffix;
  let keyword = &root.tokens[0].value;
  let mut bare: String;

  if keyword.starts_with(",P?") {
    bare = keyword[3..].to_string();
    prefix = KeywordPrefix::Qfix;
  } else if keyword.starts_with(",") {
    bare = keyword[1..].to_string();
    prefix = KeywordPrefix::Comma;
  } else if keyword.starts_with(".") {
    bare = keyword[1..].to_string();
    prefix = KeywordPrefix::Dot;
  } else {
    bare = keyword.to_string();
    prefix = KeywordPrefix::None;
  }

  bare = bare.replace("-", "_");

  let first_char = bare.chars().next().unwrap();
  if first_char.is_numeric() {
    let alpha = match first_char {
      '0' => "zero",
      '1' => "one",
      '2' => "two",
      '3' => "three",
      '4' => "four",
      '5' => "five",
      '6' => "six",
      '7' => "seven",
      '8' => "eight",
      '9' => "nine",
      _ => return Err(OutputErr::from(HandlerErr::origin(format!("Trying to crack keyword, but first char is not numeric?? {}", root))))
    };
    bare = format!("{}{}", alpha, &bare[1..]);
  }

  if bare.ends_with("?") {
    bare = format!("{}", &bare[..(bare.len()-1)]);
    suffix = KeywordSuffix::Question;
  } else {
    suffix = KeywordSuffix::None;
  }

  if !bare.chars().all(|c| { c.is_alphanumeric() || c == '_' }) {
    return Err(OutputErr::from(HandlerErr::origin(format!("Trying to crack keyword, but bare still has symbols in it: {}", root))));
  }

  Ok((prefix, bare,  suffix))
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeywordPrefix {
  Qfix,
  Comma,
  Dot,
  None,
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeywordSuffix {
  Question,
  None,
}