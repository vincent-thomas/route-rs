use std::io;

use route::{
  html::tags::{
    head::Head,
    html::Html,
    link::{Link, LinkLoadType},
    Div, IntoTag as _,
  },
  web, App, Respondable,
};
use route_html::tags::Header;
use tokio::net::TcpListener;

fn default_head() -> Head {
  Head::default().title("testing").reset_css()
}

async fn index() -> impl Respondable {
  Html::with_head(default_head()).body_from_iter([
    Header::from([
      Div::text("testing").into_tag(),
      Div::text("testing").into_tag(),
    ])
    .styles(
      "
        display: flex;
        flex-direction: row;
        width: 100%;
        justify-content: space-between;

        padding: 0.75rem;
        ",
    )
    .into_tag(),
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

  let app = App::default().at("/", web::get(index));

  route::serve(listener, app).await
}
