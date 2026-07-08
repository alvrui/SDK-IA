// API middleware for Cadiz12 project

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct LoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService { service }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        
        // Log request
        println!("[REQUEST] {} {}", method, path);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            // Log response
            println!("[RESPONSE] {} {} -> {}", method, path, res.status());
            Ok(res)
        })
    }
}