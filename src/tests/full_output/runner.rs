use std::path::Path;
use std::io::BufRead;

#[test]
pub fn test_objects() {
  let path = Path::new(".").join("src").join("tests").join("full_output").join("test_objects");
  test_folder(&path)
}

#[test]
pub fn test_repeat() {
  let path = Path::new(".").join("src").join("tests").join("full_output").join("test_repeat");
  test_folder(&path)
}

#[test]
pub fn test_routine() {
  let path = Path::new(".").join("src").join("tests").join("full_output").join("test_routine");
  test_folder(&path)
}

#[test]
pub fn test_comments() {
  let path = Path::new(".").join("src").join("tests").join("full_output").join("test_comments");
  test_folder(&path)
}

pub fn test_folder(folder: &Path) {    
    let input_file_path = folder.join("input.zil");
    let reader = crate::get_BufReader(&input_file_path).unwrap();
    let mut generator = crate::zil::tokenize::TokenGenerator::new(0, reader);
    let mut root = crate::zil::ast::Node::new();
    crate::zil::ast::build_tree(&mut generator, &mut root).unwrap();

    let output_file_path = folder.join("actual.js");
    let writer = crate::get_CustomBufWriter(&output_file_path).unwrap();
    crate::js::parse::parse(&root, writer).unwrap();

    let mut actual = crate::get_BufReader(&output_file_path).unwrap();
    let expected_file_path = folder.join("expected.js");
    let mut expected = crate::get_BufReader(&expected_file_path).unwrap();
    
    let mut actual_end = false;
    let mut expected_end = false;

    loop {
      let mut actual_line_buf = String::new();
      let mut expected_line_buf = String::new();

      match actual.read_line(&mut actual_line_buf) {
        Ok(0) => { actual_end = true; },
        Err(_) => panic!(),
        Ok(_) => (),
      };

      match expected.read_line(&mut expected_line_buf) {
        Ok(0) => { expected_end = true },
        Err(_) => panic!(),
        Ok(_) => (),
      };

      if actual_end && expected_end {
        break;
      } else if actual_end || expected_end {
        panic!();
      }

      assert_eq!(actual_line_buf, expected_line_buf);
    }
}
