use std::path::Path;

#[test]
pub fn test_brackets_1() {
  let input_file_path = Path::new(".").join("src").join("tests").join("zil_errors").join("brackets_1.zil");
  test_file(&input_file_path)
}

#[test]
pub fn test_brackets_2() {
  let input_file_path = Path::new(".").join("src").join("tests").join("zil_errors").join("brackets_2.zil");
  test_file(&input_file_path)
}

#[test]
pub fn test_brackets_3() {
  let input_file_path = Path::new(".").join("src").join("tests").join("zil_errors").join("brackets_3.zil");
  test_file(&input_file_path)
}

#[test]
pub fn test_comment_1() {
  let input_file_path = Path::new(".").join("src").join("tests").join("zil_errors").join("comment_1.zil");
  test_file(&input_file_path)
}

#[test]
pub fn test_string_1() {
  let input_file_path = Path::new(".").join("src").join("tests").join("zil_errors").join("string_1.zil");
  test_file(&input_file_path)
}

#[test]
pub fn test_string_2() {
  let input_file_path = Path::new(".").join("src").join("tests").join("zil_errors").join("string_2.zil");
  test_file(&input_file_path)
}

pub fn test_file(input_file_path: &Path) {    
    let reader = crate::get_BufReader(&input_file_path).unwrap();
    let mut generator = crate::zil::tokenize::TokenGenerator::new(0, reader);
    let mut root = crate::zil::ast::Node::new();
    match crate::zil::ast::build_tree(&mut generator, &mut root) {
      Ok(()) => panic!(),
      Err(_) => ()
    };
}