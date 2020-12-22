use std::path::Path;

#[test]
#[allow(non_snake_case)]
pub fn test_COND_1() {
  let input_file_path = Path::new(".").join("src").join("tests").join("js_errors").join("bad_COND_1.zil");
  test_folder(&input_file_path)
}

#[test]
#[allow(non_snake_case)]
pub fn test_COND_2() {
  let input_file_path = Path::new(".").join("src").join("tests").join("js_errors").join("bad_COND_2.zil");
  test_folder(&input_file_path)
}

#[test]
#[allow(non_snake_case)]
pub fn test_ROUTINE_1() {
  let input_file_path = Path::new(".").join("src").join("tests").join("js_errors").join("bad_ROUTINE_1.zil");
  test_folder(&input_file_path)
}

#[test]
#[allow(non_snake_case)]
pub fn test_SET_1() {
  let input_file_path = Path::new(".").join("src").join("tests").join("js_errors").join("bad_SET_1.zil");
  test_folder(&input_file_path)
}

pub fn test_folder(input_file_path: &Path) {    
    let reader = crate::get_BufReader(&input_file_path).unwrap();
    let mut generator = crate::zil::tokenize::TokenGenerator::new(0, reader);
    let mut root = crate::zil::ast::Node::new();
    crate::zil::ast::build_tree(&mut generator, &mut root).unwrap();

    let output_file_path = Path::new(".").join("out").join("testing.js");
    let writer = crate::get_CustomBufWriter(&output_file_path).unwrap();
    match crate::js::parse::parse(&root, writer) {
      Ok(()) => panic!(),
      Err(_) => ()
    };
}
