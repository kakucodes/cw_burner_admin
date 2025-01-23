#[cfg(any(not(feature = "library"), not(target_arch = "wasm32")))]
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;
#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
mod interface;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::BurnerAdminContract;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::msg::{ExecuteMsgFns as BurnerAdminExecuteMsgFns, QueryMsgFns as BurnerAdminQueryMsgFns};
