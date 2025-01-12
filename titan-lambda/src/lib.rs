mod lambda_handler_service;
use std::future::Future;

pub use lambda_http::Request;

pub use lambda_handler_service::LambdaHandlerService;
use titan::{lambda::LambdaAppService, App};
use titan_core::{FromRequest, Handler, Respondable};

/// Wraps an async handler in a runtime environment for processing AWS Lambda events.
///
/// This function creates a [`LambdaHandlerService`] that integrates the provided handler with an AWS Lambda-compatible runtime.
/// The handler is an async function responsible for processing incoming requests and generating responses.
///
/// # Type Parameters
/// - `H`: The type of the handler. It must be a function or type that implements the [`Handler`] trait.
/// - `Args`: The type representing the arguments passed to the handler. It must implement the [`FromRequest`] trait.
///
/// # Parameters
/// - `handler`: An async function or handler type that processes requests in the Lambda runtime. The handler should
///   be an async function or struct that implements the [`Handler`] trait.
///
/// # Constraints
/// - `H`:
///   - Must implement the [`Handler`] trait for handling requests of type `Args`.
///   - Must be [`Clone`] to allow it to be used concurrently for multiple requests.
/// - `H::Future`:
///   - Must implement [`Future`] with an output type corresponding to the handler’s response.
///   - Must be [`Send`] to ensure it can be used across threads safely.
/// - `H::Output`:
///   - Must implement [`Respondable`], allowing the handler's output to be transformed into a valid Lambda response.
/// - `Args`:
///   - Must implement the [`FromRequest`] trait to handle the conversion of incoming requests into the `Args` type.
///   - Must be both [`Send`] and [`Sync`] to ensure safe concurrent access and use in the Lambda environment.
///   - Must have a `'static` lifetime to ensure it does not contain any non-static references.
/// - `Args::Error`:
///   - Must be [`Send`] to ensure errors can be safely sent across threads.
///
/// # Returns
/// A [`LambdaHandlerService`] instance that processes incoming Lambda events using the provided handler.
///
/// # Examples
///
/// ```rust
/// use titan::{web, Respondable};
///
/// async fn my_handler(body_str: String) -> impl Respondable {
///     "Hello World"
/// }
///
/// #[tokio::main]
/// async fn main() {
///   // Uncomment the last line to run example
///   titan_lambda::handler_runtime(my_handler).run(); // .await.unwrap();
/// }
/// ```
///
/// # See Also
/// - [`Handler`]: For the handler trait that your async function must implement.
/// - [`FromRequest`]: For handling the conversion of incoming data into request types.
/// - [`Respondable`]: For handling the conversion of response data into a Lambda-compatible format.
/// - [`LambdaHandlerService`]: For the service that integrates the handler with AWS Lambda.
///
/// # Errors
/// Any errors that arise during request processing or handler execution will be propagated through the [`LambdaHandlerService`].
pub fn handler_runtime<H, Args>(handler: H) -> LambdaHandlerService<H, Args>
where
  H: Handler<Args> + Clone,
  H::Future: Future<Output = H::Output> + Send,
  H::Output: Respondable,
  Args: FromRequest + Send + Sync + 'static,
  Args::Error: Send,
{
  LambdaHandlerService::new(handler)
}

/// Wraps an [`App`] in a runtime environment compatible with AWS Lambda.
///
/// This function sets up a [`LambdaAppService`] to process events in an AWS Lambda runtime,
/// using the provided [`App`] instance. It simplifies the integration of applications with Lambda
/// by providing a service that manages incoming requests and outgoing responses.
///
/// # Parameters
/// - `app`: An instance of [`App`] that represents the application to be run in the Lambda environment.
///
/// # Returns
/// A [`LambdaAppService`] that can process incoming Lambda events using the provided application.
///
/// # Examples
///
/// ```rust
/// use titan::App;
///
/// #[tokio::main]
/// async fn main() {
///   let app = App::default(); // Empty app example
///
///   // Uncomment last line to run this example
///   titan_lambda::app_runtime(app).run(); // .await.unwrap();
/// }
/// ```
///
/// # See Also
/// - [`App`]: For implementing the application logic.
/// - [`LambdaAppService`]: For the service that integrates with AWS Lambda.
///
/// # Errors
/// Any errors encountered during the processing of requests are handled and propagated
/// by the [`LambdaAppService`].
pub fn app_runtime(app: App) -> LambdaAppService {
  LambdaAppService::new(app)
}
