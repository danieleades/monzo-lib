use crate::{client::request::Request, Result};
use chrono::{DateTime, Utc};
use pin_project::pin_project;
use reqwest::RequestBuilder;
use serde::Deserialize;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Deserialize, Debug)]
pub struct Accounts {
    accounts: Vec<Account>,
}

impl std::ops::Deref for Accounts {
    type Target = Vec<Account>;
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

#[pin_project]
pub struct AccountsRequest {
    #[pin]
    request: Request<Accounts>,
}

impl From<RequestBuilder> for AccountsRequest {
    fn from(request_builder: RequestBuilder) -> Self {
        let request = request_builder.into();
        AccountsRequest { request }
    }
}

impl Future for AccountsRequest {
    type Output = Result<Accounts>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().request.poll(cx)
    }
}
