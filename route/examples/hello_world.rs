use std::io;

use route::{web, App, Respondable};
use route_html::tags::{
  head::Head,
  html::Html,
  link::{Link, LinkLoadType},
  Div, IntoTag as _,
};
use tokio::net::TcpListener;

fn default_head() -> Head {
  Head::default().title("testing").reset_css()
}

async fn index() -> impl Respondable {
  Html::with_head(default_head()).body_from_iter([
    Link::text("/about", "About me")
      .preload(LinkLoadType::WhenIdle)
      .styles(
        "
            color: blue;
            background-color: red;
          ",
      )
      .into_tag(),
    Link::text("/".to_string(), "testing")
      .preload(LinkLoadType::WhenHover)
      .styles(
        "
            color: blue;
            background-color: red;
          ",
      )
      .into_tag(),
  ])
}

async fn about_me() -> impl Respondable {
  Html::with_head(default_head()).body_from_iter([
    Div::default().into_tag(),
    Link::text("/", "testing")
      .preload(LinkLoadType::WhenIdle)
      .styles(
        "
            color: blue;
            background-color: red;
          ",
      )
      .into_tag(),
    Link::text("/about".to_string(), "testing")
      .preload(LinkLoadType::WhenHover)
      .styles(
        "
            color: blue;
            background-color: red;
          ",
      )
      .into_tag(),
  ])
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

  let app =
    App::default().at("/", web::get(index)).at("/about", web::get(about_me));

  route::serve(listener, app).await
}
