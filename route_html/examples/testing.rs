use route_html::tags::head::Head;
use route_html::tags::html::Html;
use route_html::tags::link::{Link, LinkLoadType};
use route_html::tags::IntoTag as _;

fn main() {
  let root =
    Html::with_head(Head::default().title("testing")).body_from_iter([
      Link::text("http://localhost:3000", "testing")
        .preload(LinkLoadType::WhenIdle)
        .styles(
          "
            color: blue;
            background-color: red;
          ",
        )
        .into_tag(),
      Link::text("http://localhost:3000".to_string(), "testing")
        .preload(LinkLoadType::WhenHover)
        .styles(
          "
            color: blue;
            background-color: red;
          ",
        )
        .into_tag(),
    ]);

  println!("{}", route_html::render(root));
}
