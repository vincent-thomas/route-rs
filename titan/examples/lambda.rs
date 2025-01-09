use lambda_http::run;
use titan::deploy::lambda::wrap_lambda;
use titan::Respondable;

async fn testing() -> impl Respondable {
  "testing"
}

#[tokio::main]
async fn main() {
  run(wrap_lambda(testings)).await
}
