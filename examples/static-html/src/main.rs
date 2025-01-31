use std::io;
use titan::App;

use titan::{
  html::{
    css, global_css,
    tags::{head::Head, html::Html, *},
    StyleRule,
  },
  http::Respondable,
  web,
};

const LINK_CSS: &[StyleRule] = css!(
  "
  color: blue;
  padding: 0.55;
  background-color: red;
"
);

const TESTING: &[StyleRule] = css!(
  "
  display: flex;
  flex-direction: row;
  width: 100%;
  justify-content: space-between;

  padding: 0.75rem;
"
);

#[titan::ssg]
async fn index() -> impl Respondable {
  Html::from((
    Head::default().global_style(global_css!("")).reset_css(),
    Body::default().children([
      Header::default()
        .styles(TESTING)
        .children([
          Div::text("testing").into_tag(),
          Div::default().children([P::text("testing").into_tag()]).into_tag(),
        ])
        .into_tag(),
      Link::text("/", "testing")
        .preload(LinkLoadType::WhenIdle)
        .styles(LINK_CSS)
        .into_tag(),
      Link::text("/about", "testing")
        .preload(LinkLoadType::WhenIdle)
        .styles(LINK_CSS)
        .into_tag(),
      Script::from_text("console.log(\"Hello World!\");").into_tag(),
    ]),
  ))
  .with_csp("examplenonce")
}

#[titan::ssg]
fn testing() -> Html {
  println!("ran");
  Html::from((Head::default(), Body::default()))
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let app = App::default()
    .at("/", web::get(index))
    .at("/test/testing", web::get(testing));

  titan::build_static(app, std::path::PathBuf::from("./dist")).await;

  Ok(())
}
