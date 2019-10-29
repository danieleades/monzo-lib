use std::future::Future;
pub trait IntoFuture {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn into_future(self) -> Self::Future;
}
