use crate::zil::ast::*;

pub fn format_string(root: &Node) -> Result<String, ()> {
  if !root.is_text() && !root.is_word() {
      return Err(());
  }

  let escaped = root.tokens[0].value.replace("\"", "\\\"");
  Ok(String::from(format!("\"{}\"", escaped)))
}

pub fn format_keyword(root: &Node) -> Result<String, ()> {
  let (mut keyword, prefix) = crack_keyword(&root)?;

  keyword = match prefix {
    KeywordWrapper::Pfix => format!("pq_{}", &keyword),
    KeywordWrapper::Comma => format!("comma_{}", &keyword),
    KeywordWrapper::Dot => format!("dot_{}", &keyword),
    KeywordWrapper::None => keyword
  };

  Ok(String::from(keyword))
}

pub fn crack_keyword(root: &Node) -> Result<(String, KeywordWrapper), ()> {
  if !root.is_word() {
    return Err(());
  }

  let mut bare: String;
  let prefix: KeywordWrapper; 
  let keyword = &root.tokens[0].value;

  if keyword.starts_with(",P?") {
    bare = keyword[3..].to_string();
    prefix = KeywordWrapper::Pfix;
  } else if keyword.starts_with(",") {
    bare = keyword[1..].to_string();
    prefix = KeywordWrapper::Comma;
  } else if keyword.starts_with(".") {
    bare = keyword[1..].to_string();
    prefix = KeywordWrapper::Dot;
  } else {
    bare = keyword.to_string();
    prefix = KeywordWrapper::None;
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
      _ => return Err(())
    };
    bare = format!("{}{}", alpha, &bare[1..]);
  }

  if bare.ends_with("?") {
    bare = format!("{}{}", &bare[..(bare.len()-1)], "_q");
  }

  if bare.contains(",") ||
     bare.contains(".") ||
     bare.contains("?") {
    return Err(());
  }

  Ok((bare, prefix))
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeywordWrapper {
  Pfix,
  Comma,
  Dot,
  None,
}