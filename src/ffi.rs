
use search_context::*;

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn search_context_new() -> *mut SearchContext {
    Box::into_raw(Box::new(SearchContext::new()))
}

#[no_mangle]
pub extern "C" fn search_context_free(ptr: *mut SearchContext) {
    if !ptr.is_null() { unsafe { Box::from_raw(ptr); } }
}

#[no_mangle]
pub extern "C" fn search_context_fuzzy_search(ptr: *mut SearchContext, pattern: *const c_char) -> Vec<usize> {
    let ref context = unsafe { &*ptr };
    let pattern = unsafe { CStr::from_ptr(pattern).to_string_lossy() };

    context.fuzzy_search(pattern)
}