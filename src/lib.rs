#[allow(non_camel_case_types)] // bindgen生成的type alias不符合rust规范
pub mod bind;
pub mod error;

use std::{
    convert::AsRef,
    ffi::{c_void, CStr, CString},
    ptr,
};

use error::Error;

pub type Address = *mut c_void;

/// 替换一个函数调用
///
/// Replace a function call
///
/// # Safety
///
/// Hook可以引起各种未定义的行为，因此不安全
///
/// Hook can cause all kinds of undefined behavior, so it's unsafe
pub unsafe fn hook(
    target_func_addr: Address,
    replace_func_addr: Address,
    ori_func_save: Option<&mut Address>,
) -> Result<(), Error> {
    let result = if let Some(save_ptr) = ori_func_save {
        bind::DobbyHook(target_func_addr, replace_func_addr, save_ptr)
    } else {
        bind::DobbyHook(target_func_addr, replace_func_addr, ptr::null_mut())
    };

    if result == -1 {
        return Err(Error::FailedToHook(target_func_addr, replace_func_addr));
    }

    Ok(())
}

/// 在指定的地址解除Hook
///
/// Release the Hook at the specified address
///
/// # Safety
///
/// Hook可以引起各种未定义的行为，因此不安全
///
/// Hook can cause all kinds of undefined behavior, so it's unsafe
pub unsafe fn undo_hook(undo_addr: Address) -> Result<(), Error> {
    if bind::DobbyDestroy(undo_addr) == -1 {
        Err(Error::FailedToUndoHook(undo_addr))
    } else {
        Ok(())
    }
}

/// 通过指定的符号搜索一个函数的地址
///
/// Search for the address of a function by the specified symbol
///
/// library: 库名(可以为None)
///
/// symbol: 符号名(建议: 尝试readelf -s /path/to/elf)
///
/// library: lib name(Or None)
///
/// symbol: symbol name(sugg. Try readelf -s /path/to/elf)
pub fn resolve_func_addr<S: AsRef<str>>(library: Option<S>, symbol: S) -> Result<Address, Error> {
    let symbol = symbol.as_ref();

    let addr = unsafe {
        let symbol = CString::new(symbol).unwrap();

        if let Some(library) = library {
            let library = CString::new(library.as_ref()).unwrap();
            bind::DobbySymbolResolver(library.as_ptr(), symbol.as_ptr())
        } else {
            bind::DobbySymbolResolver(ptr::null(), symbol.as_ptr())
        }
    };

    if addr.is_null() {
        Err(Error::FuncNotFound(symbol.to_string()))
    } else {
        Ok(addr)
    }
}

/// 获取`DobbyVersion`
///
/// Get`DobbyVersion`
pub fn get_version() -> &'static str {
    unsafe { CStr::from_ptr(bind::DobbyGetVersion()).to_str().unwrap() }
}
