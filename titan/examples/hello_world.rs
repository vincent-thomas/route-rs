use std::io;

use titan::{
  html::tags::{
    head::Head,
    html::Html,
    link::{Link, LinkLoadType},
    Body, Div, Header, IntoTag as _, P,
  },
  web, App, Respondable,
};
use titan_html::{css, global_css, StyleRule};
use tokio::net::TcpListener;

//fn default_head() -> Head {
//  Head::default().title("testing").reset_css()
//}

fn link_css() -> Vec<StyleRule> {
  css!(
    "
    color: blue;
    padding: 0.55;
    background-color: red;
"
  )
}

async fn index(body: String) -> impl Respondable {
  let testing = css!(
    "
  display: flex;
  flex-direction: row;
  width: 100%;
  justify-content: space-between;

  padding: 0.75rem;
      "
  );
  Html::from((
    Head::default().global_style(global_css!("")),
    Body::default().children([
      Header::default()
        .styles(testing)
        .children([
          Div::text("testing").into_tag(),
          Div::default().children([P::text("testing").into_tag()]).into_tag(),
        ])
        .into_tag(),
      Link::text("/", "testing")
        .preload(LinkLoadType::WhenIdle)
        .styles(link_css())
        .into_tag(),
      Link::text("/about".to_string(), "testing")
        .preload(LinkLoadType::WhenIdle)
        .styles(link_css())
        .into_tag(),
      //Div::text(body).into_tag(),
    ]),
  ))
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

  let app = App::default().at("/", web::get(index));

  titan::serve(listener, app).await
}
