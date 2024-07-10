use criterion::{black_box, criterion_group, criterion_main, Criterion};
use route_http::{method::HttpMethod, request::HttpRequest, response::Respondable};
use route_router::{Route, Router};

async fn test(_: HttpRequest) -> impl Respondable {
  "test".to_string()
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut router = Router::mount_at("/");

  router.route(HttpMethod::Get, "/user/test".to_string(), Route::new(test));
  // router.route(HttpMethod::Post, "/user/{user_id}".to_string(), Route::new(&test2));

  //     let output = router.match_route(RouteMethod::Post, "/user/testing");
  c.bench_function("router_match", |b| {
    b.iter(|| router.match_route(black_box(HttpMethod::Post), black_box("/users/testing")))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
