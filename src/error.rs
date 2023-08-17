use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to apply hook at target address")]
    FailedToHook,
    #[error("Failed to undo hook at address")]
    FailedToUndoHook,
    #[error("Failed to find func by symbol")]
    FuncNotFound,
    #[error("Memory error happend, error type: MemoryOperationError")]
    MemoryOperationError,
    #[error("Memory error happend, error type: NotSupportAllocateExecutableMemory")]
    NotSupportAllocateExecutableMemory,
    #[error("Memory error happend, error type: MemoryOperationErrorNotEnough")]
    MemoryOperationErrorNotEnough,
    #[error("Memory error happend, error type: MemoryOperationErrorNone")]
    MemoryOperationErrorNone,
}
