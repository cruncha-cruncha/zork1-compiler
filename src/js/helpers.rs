use crate::zil::contracts::ZilNode;
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

pub fn escape_text(root: &ZilNode) -> Result<String, OutputErr> {
  if !root.is_text() {
      return Err(OutputErr::from(HandlerErr::origin(format!("bad text to escape: {}", root))));
  }

  let escaped = root.tokens[0].value.replace("\"", "\\\"").replace("\n", "\\n");
  Ok(String::from(escaped))
}

pub fn is_int(root: &ZilNode) -> bool {
  if !root.is_word() {
    return false;
  }

  match root.tokens[0].value.parse::<usize>() { // turbofish!
    Ok(_) => true,
    Err(_) => false
  }
}

pub fn format_keyword(root: &ZilNode) -> Result<String, OutputErr> {
  if !root.is_word() && !root.is_text() {
    return Err(OutputErr::from(HandlerErr::origin(format!("invalid keyword to format: {}", root))));
  }

  // overrides
  match &root.tokens[0].value[..] {
    "T" => return Ok(String::from("true")),
    "0?" => return Ok(String::from("isZero")),
    "1?" => return Ok(String::from("isOne")),
    _ => (),
  }

  let (prefix, mut keyword, suffix) = wrap!(crack_keyword(&root), root);

  keyword = match prefix {
    KeywordPrefix::Qfix((b, s)) => {
      match *b {
        KeywordPrefix::Comma => format!("comma_{}q_{}", s, &keyword),
        KeywordPrefix::Dot => format!("dot_{}q_{}", s, &keyword),
        KeywordPrefix::None =>format!("{}q_{}", s, &keyword),
        _ => return Err(OutputErr::from(HandlerErr::origin(format!("Cannot format unknown KeywordPrefix in Qfix of format_keyword {}", root))))
      }
    },
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

pub fn crack_keyword(root: &ZilNode) -> Result<(KeywordPrefix, String, KeywordSuffix), OutputErr> {
  if !root.is_word() && !root.is_text() {
    return Err(OutputErr::from(HandlerErr::origin(format!("invalid keyword to crack: {}", root))));
  }

  let prefix: KeywordPrefix;
  let suffix: KeywordSuffix;
  let mut bare = String::from(&root.tokens[0].value);

  bare = bare.replace("-", "_");

  if bare.ends_with("?") {
    bare = format!("{}", &bare[..(bare.len()-1)]);
    suffix = KeywordSuffix::Question;
  } else {
    suffix = KeywordSuffix::None;
  }

  if bare.contains("?") {
    let i = bare.find("?").unwrap();
    let pre = &bare[..i];
    if pre.starts_with(",") {
      prefix = KeywordPrefix::Qfix((Box::new(KeywordPrefix::Comma), String::from(&pre[1..])));
    } else if pre.starts_with(".") {
      prefix = KeywordPrefix::Qfix((Box::new(KeywordPrefix::Dot), String::from(&pre[1..])));
    } else {
      prefix = KeywordPrefix::Qfix((Box::new(KeywordPrefix::None), String::from(pre)));
    }
    bare = bare[(i+1)..].to_string();
  } else if bare.starts_with(",") {
    prefix = KeywordPrefix::Comma;
    bare = bare[1..].to_string();
  } else if bare.starts_with(".") {
    prefix = KeywordPrefix::Dot;
    bare = bare[1..].to_string();
  } else {
    prefix = KeywordPrefix::None;
    bare = bare.to_string();
  }

  if !bare.chars().all(|c| { c.is_alphanumeric() || c == '_' }) {
    return Err(OutputErr::from(HandlerErr::origin(format!("Trying to crack keyword, but bare still has symbols in it: {}", root))));
  }

  Ok((prefix, bare,  suffix))
}

#[derive(Clone, PartialEq)]
pub enum KeywordPrefix {
  Qfix((Box<KeywordPrefix>, String)),
  Comma,
  Dot,
  None,
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeywordSuffix {
  Question,
  None,
}