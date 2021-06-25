//! 'Inner' clients that encapsulate the different authentication handling
//! strategies that the Monzo API supports.

mod quick;
mod refreshable;

pub use quick::Quick;
pub use refreshable::Refreshable;
