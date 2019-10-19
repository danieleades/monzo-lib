use crate::{Error, Result};
use serde::de::DeserializeOwned;
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

pub mod accounts;
pub mod auth;
pub mod balance;
pub mod pots;

type BoxFuture<Response> = Pin<Box<dyn Future<Output = Response>>>;

/// An http request to the Monzo API
///
/// This
pub struct RequestBuilder<Request, Response> {
    reqwest_builder: Option<reqwest::RequestBuilder>,
    inner_future: Option<BoxFuture<Result<Response>>>,
    request_type: PhantomData<Request>,

    #[cfg(debug_assertions)]
    has_resolved: bool,
}

impl<Request, Response> Unpin for RequestBuilder<Request, Response> {}

impl<Request, Response> From<reqwest::RequestBuilder> for RequestBuilder<Request, Response> {
    fn from(reqwest_builder: reqwest::RequestBuilder) -> Self {
        RequestBuilder {
            reqwest_builder: Some(reqwest_builder),
            inner_future: None,
            request_type: PhantomData::default(),

            #[cfg(debug_assertions)]
            has_resolved: false,
        }
    }
}

impl<Request, Response> RequestBuilder<Request, Response> {
    pub(crate) fn reqwest_builder(&mut self) -> &mut reqwest::RequestBuilder {
        self.reqwest_builder.as_mut().unwrap()
    }
}

impl<Request, Response> Future for RequestBuilder<Request, Response>
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
            let request_builder = this.reqwest_builder.take().unwrap();

            this.inner_future = Some(Box::pin(async move {
                let response = request_builder.send().await?;

                match response.status() {
                    x if x.is_success() => Ok(response.json().await?),
                    x if x.is_client_error() || x.is_server_error() => Err(Error::from(x)),
                    _ => unreachable!(),
                }
            }))
        }

        this.inner_future.as_mut().unwrap().as_mut().poll(cx)
    }
}
