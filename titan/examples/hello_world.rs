use std::io;

use titan::{
  html::{
    css,
    tags::{
      head::Head,
      html::Html,
      link::{Link, LinkLoadType},
      Body, Div, Header, IntoTag as _, P,
    },
  },
  web, App, Respondable,
};
use tokio::net::TcpListener;

fn default_head() -> Head {
  Head::default().title("testing").reset_css()
}

async fn index() -> impl Respondable {
  Html::from((
    default_head(),
    Body::from([
      Header::from([
        Div::text("testing").into_tag(),
        Div::from([P::text("testing").into_tag()]).into_tag(),
      ])
      .styles(css!(
        "
        display: flex;
        flex-direction: row;
        width: 100%;
        justify-content: space-between;

        padding: 0.75rem;
        "
      ))
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
        .preload(LinkLoadType::WhenIdle)
        .styles(
          "
            color: blue;
            background-color: red;
          ",
        )
        .into_tag(),
    ]),
  ))
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

  let app = App::default().at("/", web::get(index));

  titan::serve(listener, app).await
}
