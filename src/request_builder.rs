use crate::{
    client,
    endpoints::{Endpoint, Resolve},
    Error, Result,
};

/// A wrapper around a type that implements [`Endpoint`] and [`Resolve`] that
/// handles making requests to the Monzo API.
#[derive(Debug)]
#[must_use]
pub struct RequestBuilder<'a, C, E>
where
    E: Endpoint,
    C: client::Inner,
{
    client: &'a C,
    endpoint: E,
}

impl<'a, C, E> RequestBuilder<'a, C, E>
where
    E: Endpoint + Resolve,
    C: client::Inner,
{
    pub(crate) fn new(client: &'a C, endpoint: E) -> Self {
        Self { client, endpoint }
    }

    pub(crate) fn endpoint_ref_mut(&mut self) -> &mut E {
        &mut self.endpoint
    }

    /// Consume the [`RequestBuilder`] and return the result
    pub async fn send(self) -> Result<E::Response> {
        let response = self
            .client
            .execute(&self.endpoint)
            .await
            .map_err(Error::Http)?;

        let status = response.status();

        if status.is_success() {
            Ok(self.endpoint.resolve(&response.bytes().await?)?)
        } else {
            Err(Error::from(status))
        }
    }
}
