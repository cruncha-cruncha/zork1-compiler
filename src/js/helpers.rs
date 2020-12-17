use crate::zil::ast::*;

pub fn format_string(root: &Node) -> Result<String, ()> {
  if !root.is_text() && !root.is_word() {
      return Err(());
  }

  let escaped = root.tokens[0].value.replace("\"", "\\\"");
  Ok(String::from(format!("\"{}\"", escaped)))
}

pub fn format_keyword(root: &Node) -> Result<String, ()> {
  if !root.is_word() {
      return Err(());
  }

  let keyword = &root.tokens[0].value.replace("-", "_").replace(".", "_2").replace(",", "_3").replace("?", "_q");
  Ok(String::from(keyword))
}

pub fn crack_keyword(root: &Node) -> Result<(String, KeywordWrapper), ()> {
  if !root.is_word() {
    return Err(());
  }

  let bare: String;
  let prefix: KeywordWrapper; 
  let keyword = &root.children[0].tokens[0].value;

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

  if bare.contains(",") ||
     bare.contains(".") ||
     bare[..(bare.len()-1)].contains("?") {
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