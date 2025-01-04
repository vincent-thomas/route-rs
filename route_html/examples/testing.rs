use route_html::link::{Link, LinkLoadType};
use route_html::style::StyleRule;
use route_html::tag::Html;
use route_html::tag::IntoTag as _;

fn main() {
  let link = Link::new("//google.com".to_string(), vec![Box::new("testing")])
    .preload(LinkLoadType::WhenIdle);

  let style_rule = StyleRule::from_iter([
    ("color".to_string(), "black".to_string()),
    ("background-color".to_string(), "black".to_string()),
  ]);

  let mut root = Html::default();
  let mut link = link.into_tag();

  link.get_mut(0).unwrap().style(style_rule);
  root.extend(link);

  println!("{}", route_html::render(root));
}
