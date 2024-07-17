use criterion::{black_box, criterion_group, criterion_main, Criterion};
use route_http::{method::Method, request::HttpRequest};
use route_router::Router;

fn criterion_benchmark(c: &mut Criterion) {
  let mut router = Router::new();

  router.route("/user/test", "hej");
  router.route("/user/{user_id}", "hej");

  //     let output = router.match_route(RouteMethod::Post, "/user/testing");
  c.bench_function("router_match", |b| {
    b.iter(|| router.at(black_box("/user/test"), black_box("/users/testing")))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
