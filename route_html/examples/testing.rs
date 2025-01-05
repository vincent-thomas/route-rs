use route_html::tags::head::opengraph::{OpenGraph, OpenGraphType};
use route_html::tags::head::Head;
use route_html::tags::html::Html;
use route_html::tags::link::{Link, LinkLoadType};

fn main() {
  let og = OpenGraph::new(
    "testing",
    "nice nice",
    OpenGraphType::Website,
    "http://localhost:3000",
    "http//localhost:3000/image.jpg",
  );
  let root =
    Html::with_head(Head::default().title("testing").opengraph(og).reset_css())
      .body_from_iter([
        Link::text("http://localhost:3000", "testing")
          .preload(LinkLoadType::WhenIdle)
          .styles(
            "
            color: blue;
            background-color: red;
          ",
          ),
        Link::text("http://localhost:3000".to_string(), "testing")
          .preload(LinkLoadType::WhenHover)
          .styles(
            "
            color: blue;
            background-color: red;
          ",
          ),
      ]);

  println!("{}", route_html::render(root));
}
