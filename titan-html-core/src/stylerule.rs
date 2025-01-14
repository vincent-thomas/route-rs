use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StyleRule {
  pub rule: &'static str,
  pub styles: &'static [(&'static str, &'static str)],
}

impl ToString for StyleRule {
  fn to_string(&self) -> String {
    let styles = self
      .styles
      .iter()
      .map(|(key, value)| format!("{key}:{value};"))
      .collect::<Vec<String>>()
      .join("");

    let total = format!(".{}{{{}}}", self.rule, styles);

    total
  }
}
