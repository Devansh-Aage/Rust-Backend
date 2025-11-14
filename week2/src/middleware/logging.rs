use std::{pin::Pin, time::Instant};

use axum::{http::Request, response::Response};
use tower::{self, Layer, Service};

#[derive(Clone)]
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    //Layer: Wrapper for our middleware
    type Service = LoggingMiddleware<S>;
    //Service is a async fn which takes a Req and returns a Res 
    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response> + Clone + Send + 'static,
    //Service(Handler) accepting a Request<ReqBody> and returning a Response 
    //Clone: in call fn we clone inner(handler)
    //Send: inner can be safely moved across threads
    //'static: inner must not have temp refs
    S::Future: Send + 'static, //future returned by our handler
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>> + Send>>;
        //async futures return unique type that cant be named but Tower requires named return type
        //Box the future with a named type 
        //we use box bcs we need to own the future + Send + type erasure(overrides the old type)
        //Future polling needs &mut self as it mutates state when it polls
        //Pin the box pointer so the future isnt moved after creation

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let mut inner = self.inner.clone();

        let start = Instant::now();

        let fut = inner.call(req);

        Box::pin(async move {
            let response = fut.await?;

            let status = response.status();
            let elapsed = start.elapsed().as_millis();

            println!("<- {} {}\n-> {} ({} ms)", method, uri, status, elapsed);

            Ok(response)
        })
    }
}
