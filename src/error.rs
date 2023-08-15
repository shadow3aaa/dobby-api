use thiserror::Error;

use super::Address;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to apply hook at target func address: {0:?} replace func address: {1:?}")]
    FailedToHook(Address, Address),
    #[error("Failed to undo hook at address: {0:?}")]
    FailedToUndoHook(Address),
    #[error("Failed to find func by symbol {0}")]
    FuncNotFound(String),
}
