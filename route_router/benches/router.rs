use criterion::{black_box, criterion_group, criterion_main, Criterion};
use route_http::{method::Method, request::HttpRequest};
use route_router::{Route, Router};

async fn test2(_: HttpRequest) -> String {
  "test".to_string()
}

async fn test(_: HttpRequest) -> String {
  "test".to_string()
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut router = Router::mount_at("/");

  router.route(Method::GET, "/user/test".to_string(), Route::new(test));
  router.route(Method::POST, "/user/{user_id}".to_string(), Route::new(test2));

  //     let output = router.match_route(RouteMethod::Post, "/user/testing");
  c.bench_function("router_match", |b| {
    b.iter(|| router.match_route(black_box(Method::POST), black_box("/users/testing")))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
