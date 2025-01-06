use titan_html::tags::{
  head::Head,
  html::Html,
  link::{Link, LinkLoadType},
  IntoTag as _,
};

fn main() {
  let root = Html::with_head(Head::empty()).body_from_iter([
    Link::text("http://localhost:3000", "testing")
      .preload(LinkLoadType::WhenHover)
      .styles(
        "
          color: blue;
          background-color: red;
        ",
      )
      .into_tag(),
    Link::text("http://localhost:3000".to_string(), "testing")
      .preload(LinkLoadType::WhenHover)
      //.styles(
      //  "
      //    color: blue;
      //    background-color: red;
      //  ",
      //)
      .into_tag(),
  ]);

  println!("{}", titan_html::render(root));
}
