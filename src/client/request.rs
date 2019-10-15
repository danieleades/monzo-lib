use crate::Result;
use pin_project::pin_project;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

type BoxFuture<Response> = Pin<Box<dyn Future<Output = Response>>>;

//#[pin_project]
pub(crate) struct Request<Response> {
    request_builder: Option<RequestBuilder>,
    inner_future: Option<BoxFuture<Result<Response>>>,

    #[cfg(debug_assertions)]
    has_resolved: bool,
}

impl<Response> Unpin for Request<Response> {}

impl<Response> From<RequestBuilder> for Request<Response> {
    fn from(request_builder: RequestBuilder) -> Self {
        Request {
            request_builder: Some(request_builder),
            inner_future: None,

            #[cfg(debug_assertions)]
            has_resolved: false,
        }
    }
}

impl<Response> std::ops::Deref for Request<Response> {
    type Target = RequestBuilder;

    fn deref(&self) -> &Self::Target {
        self.request_builder.as_ref().unwrap()
    }
}

impl<Response> std::ops::DerefMut for Request<Response> {
    fn deref_mut(&mut self) -> &mut RequestBuilder {
        self.request_builder.as_mut().unwrap()
    }
}

impl<Response> Future for Request<Response>
where
    Response: DeserializeOwned,
{
    type Output = Result<Response>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        debug_assert!(
            !this.has_resolved,
            "Cannot poll future after it is resolved!"
        );

        if this.inner_future.is_none() {
            let request_builder = this.request_builder.take().unwrap();

            this.inner_future = Some(Box::pin(async move {
                Ok(request_builder.send().await?.json().await?)
            }))
        }

        this.inner_future.as_mut().unwrap().as_mut().poll(cx)
    }
}
