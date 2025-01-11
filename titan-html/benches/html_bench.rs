use criterion::{black_box, criterion_group, criterion_main, Criterion};
use titan_html::{
  render,
  tags::{
    head::Head,
    html::Html,
    link::{Link, LinkLoadType},
    Body, Div, Header, IntoTag as _, P,
  },
};

pub fn criterion_benchmark(c: &mut Criterion) {
  let html = Html::from((
    Head::default(),
    Body::default().children([
      Header::default()
        .class("nice very-nice")
        .children([
          Div::text("testing").into_tag(),
          Div::default().children([P::text("testing").into_tag()]).into_tag(),
        ])
        .into_tag(),
      Link::text("/", "testing").preload(LinkLoadType::WhenIdle).into_tag(),
      Link::text("/about".to_string(), "testing")
        .class("nice very-nice")
        .preload(LinkLoadType::WhenIdle)
        .into_tag(),
      Div::text("haalåja").into_tag(),
    ]),
  ));

  c.bench_function("render html", |b| {
    b.iter(|| titan_html::render(black_box(html.clone())))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
