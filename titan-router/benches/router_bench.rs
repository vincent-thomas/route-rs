use criterion::{black_box, criterion_group, criterion_main, Criterion};
use titan_router::Router;

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut router = Router::default();

  router.at("/api/v1/auth/login", "auth-login");
  router.at("/api/v1/auth/register", "auth-register");
  router.at("/api/v1/auth/:user", "user");
  c.bench_function("correct static route", |b| {
    b.iter(|| router.lookup(black_box("/api/v1/auth/login")))
  });

  c.bench_function("correct dynamic route", |b| {
    b.iter(|| router.lookup(black_box("/api/v1/auth/very-nice-user")))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
