use route_html::head::{Head, OpenGraph, OpenGraphType};
use route_html::link::{Link, LinkLoadType};
use route_html::tag::Html;

fn main() {
  let og = OpenGraph::new(
    "testing",
    "nice nice",
    OpenGraphType::Website,
    "http://localhost:3000",
    "http//localhost:3000/image.jpg",
  );
  let mut root =
    Html::with_head(Head::default().title("testing").opengraph(og));

  root.body_from_iter([
    Link::text("http://localhost:3000", "testing")
      .preload(LinkLoadType::WhenIdle)
      .style_from_iter([("color", "black"), ("background-color", "black")]),
    Link::text("http://localhost:3000".to_string(), "testing")
      .preload(LinkLoadType::WhenHover)
      .style_from_iter([("color", "green"), ("background-color", "black")]),
  ]);

  //dbg!(&root);
  println!("{}", route_html::render(root));
}
