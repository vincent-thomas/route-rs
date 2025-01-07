use titan_html::{
  css,
  tags::{
    head::Head,
    html::Html,
    link::{Link, LinkLoadType},
    Div, IntoTag as _,
  },
};

fn main() {
  let root = Html::with_head(Head::empty()).body_from_iter([
    Link::text("http://localhost:3000", "testing")
      //.preload(LinkLoadType::WhenHover)
      .styles(
        "
          color: blue;
          background-color: red;
        ",
      )
      .add_id("testing")
      .into_tag(),
    Link::text("http://localhost:3000".to_string(), "testing")
      //.preload(LinkLoadType::WhenHover)
      .styles(
        "
          color: blue;
          background-color: red;
        ",
      )
      .into_tag(),
    Div::text("testing")
      .styles(css!(
        "
        background-color: green;
        color: blue;
              "
      ))
      .into_tag(),
  ]);

  println!("{}", titan_html::render(root));
}
