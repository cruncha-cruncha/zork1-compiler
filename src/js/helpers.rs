pub fn escape_text(value: &str) -> String {
  String::from(value.replace("\"", "\\\"").replace("\n", "\\n"))
}

pub fn format_keyword(value: &str) -> Result<String, ()> {
  let (prefix, mut keyword, suffix) = crack_keyword(&value)?;

  keyword = match prefix {
    KeywordPrefix::Qfix((b, s)) => {
      match *b {
        KeywordPrefix::Comma => format!("comma_{}q_{}", s, &keyword),
        KeywordPrefix::Dot => format!("dot_{}q_{}", s, &keyword),
        KeywordPrefix::None =>format!("{}q_{}", s, &keyword),
        _ => return Err(())
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

pub fn crack_keyword(value: &str) -> Result<(KeywordPrefix, String, KeywordSuffix), ()> {
  let prefix: KeywordPrefix;
  let mut bare = value.clone();
  let suffix: KeywordSuffix;

  if bare.ends_with("?") {
    bare = &bare[..(bare.len()-1)];
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
    bare = &bare[(i+1)..];
  } else if bare.starts_with(",") {
    prefix = KeywordPrefix::Comma;
    bare = &bare[1..];
  } else if bare.starts_with(".") {
    prefix = KeywordPrefix::Dot;
    bare = &bare[1..];
  } else {
    prefix = KeywordPrefix::None;
  }

  if !bare.chars().all(|c| { c.is_alphanumeric() || c == '_' }) {
    return Err(());
  }

  Ok((prefix, String::from(bare), suffix))
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