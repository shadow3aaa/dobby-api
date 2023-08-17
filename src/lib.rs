#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#[allow(non_camel_case_types)]
// The type alias generated by bindgen does not conform to the rust specification
pub mod bind;
mod error;

use std::{
    convert::AsRef,
    ffi::{c_void, CStr, CString},
    ptr,
};

pub use error::Error;

/// Memory address
pub type Address = *mut c_void;

/// Patch the code at target address with supplied bytes
///
/// # Errors
///
/// Memory Errors
///
/// # Safety
///
/// Patch may cause the program to crash, so it is unsafe
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::as_ptr_cast_mut)]
pub unsafe fn patch_code(addr: Address, code: &[u8]) -> Result<(), Error> {
    let ret = match bind::DobbyCodePatch(
        addr,
        code.as_ptr() as *mut _,
        u32::try_from(code.len()).unwrap(),
    ) {
        0 => return Ok(()),
        1 => Error::MemoryOperationError(addr),
        2 => Error::NotSupportAllocateExecutableMemory(addr),
        3 => Error::MemoryOperationErrorNotEnough(addr),
        4 => Error::MemoryOperationErrorNone(addr),
        _ => unreachable!(),
    };

    Err(ret)
}

/// Replace a function call
///
/// # Errors
///
/// Failed to apply hook
///
/// # Safety
///
/// Hook can cause all kinds of undefined behavior, so it's unsafe
pub unsafe fn hook(
    target_func_addr: Address,
    replace_func_addr: Address,
    ori_func_save: Option<&mut Address>,
) -> Result<(), Error> {
    let result = ori_func_save.map_or_else(
        || bind::DobbyHook(target_func_addr, replace_func_addr, ptr::null_mut()),
        |save_ptr| bind::DobbyHook(target_func_addr, replace_func_addr, save_ptr),
    );

    if result == -1 {
        return Err(Error::FailedToHook(target_func_addr, replace_func_addr));
    }

    Ok(())
}

/// Undo the Hook at the specified address
///
/// # Errors
///
/// Failed to undo hook
///
/// # Safety
///
/// Hook can cause all kinds of undefined behavior, so it's unsafe
pub unsafe fn undo_hook(undo_addr: Address) -> Result<(), Error> {
    if bind::DobbyDestroy(undo_addr) == -1 {
        Err(Error::FailedToUndoHook(undo_addr))
    } else {
        Ok(())
    }
}

/// Search for the address of a function by the specified symbol
///
/// library: lib name(Or None)
///
/// symbol: symbol name(sugg. Try readelf -s /path/to/elf)
///
/// It's safe to resolve a address by smybol, though hook is unsafe
///
/// # Errors
///
/// Func not found
#[allow(clippy::missing_panics_doc)]
pub fn resolve_func_addr<S: AsRef<str>>(library: Option<S>, symbol: S) -> Result<Address, Error> {
    let symbol = symbol.as_ref();

    let addr = unsafe {
        let symbol = CString::new(symbol).unwrap();

        library.map_or_else(
            || bind::DobbySymbolResolver(ptr::null(), symbol.as_ptr()),
            |library| {
                let library = CString::new(library.as_ref()).unwrap();
                bind::DobbySymbolResolver(library.as_ptr(), symbol.as_ptr())
            },
        )
    };

    if addr.is_null() {
        Err(Error::FuncNotFound(symbol.to_string()))
    } else {
        Ok(addr)
    }
}

/// Get the version of the Dobby lib, not the version of this crate
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn get_version() -> &'static str {
    unsafe { CStr::from_ptr(bind::DobbyGetVersion()).to_str().unwrap() }
}
