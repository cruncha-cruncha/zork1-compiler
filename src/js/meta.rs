pub struct Meta<'a> {
  pub indent: usize,
  pub parent: Option<&'a Meta<'a>>
}

impl Meta<'_> {
  pub fn indent<'a>(parent: &'a Meta) -> Meta<'a> {
    Meta {
      indent: parent.indent + 1,
      parent: Some(&parent)
    }
  }
}