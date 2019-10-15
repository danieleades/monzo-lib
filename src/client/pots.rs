use super::request::Request;
use crate::Result;
use chrono::{DateTime, Utc};
use pin_project::pin_project;
use reqwest::RequestBuilder;
use serde::Deserialize;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A collection of Monzo pots
#[derive(Deserialize, Debug)]
pub struct Pots {
    pots: Vec<Pot>,
}

/// Representation of a Monzo pot
#[derive(Deserialize, Debug)]
pub struct Pot {
    pub id: String,
    pub name: String,
    pub style: String,
    pub balance: i64,
    pub currency: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub deleted: bool,
}
#[pin_project]
pub(super) struct PotsRequest {
    #[pin]
    request: Request<Pots>,
}

impl From<RequestBuilder> for PotsRequest {
    fn from(request_builder: RequestBuilder) -> Self {
        let request = request_builder.into();
        Self { request }
    }
}

impl Future for PotsRequest {
    type Output = Result<Pots>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().request.poll(cx)
    }
}
