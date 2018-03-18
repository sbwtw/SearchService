
use dbus_service;
use search_context::*;

use std::slice;
use std::mem;
use std::cmp;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn start_service() {
    dbus_service::start_service();
}

#[no_mangle]
pub extern "C" fn search_context_new() -> *mut SearchContext {
    Box::into_raw(Box::new(SearchContext::new()))
}

#[no_mangle]
pub extern "C" fn search_context_free(ptr: *mut SearchContext) {
    if !ptr.is_null() { unsafe { Box::from_raw(ptr); } }
}

#[no_mangle]
pub extern "C" fn search_context_set_context(ptr: *mut SearchContext, text: *const c_char) {
    let ref mut context = unsafe { &mut *ptr };
    let text = unsafe { CStr::from_ptr(text).to_string_lossy() };

    context.set_context(text);
}

#[no_mangle]
pub extern "C" fn search_context_get_context(ptr: *const SearchContext) -> *const c_char {
    let ref context = unsafe { &*ptr };
    let s = CString::new(context.context()).unwrap();
    let p = s.as_ptr();

    mem::forget(s);

    p
}

#[no_mangle]
pub extern "C" fn search_context_search(ptr: *const SearchContext, pattern: *const c_char) -> isize {
    let ref context = unsafe { &*ptr };
    let pattern = unsafe { CStr::from_ptr(pattern).to_string_lossy() };

    context.search(pattern).map(|x| x as isize).unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn search_context_fuzzy_search(ptr: *const SearchContext, pattern: *const c_char) -> *const Vec<usize> {
    let ref context = unsafe { &*ptr };
    let pattern = unsafe { CStr::from_ptr(pattern).to_string_lossy() };

    Box::into_raw(Box::new(context.fuzzy_search(pattern)))
}

#[no_mangle]
pub extern "C" fn vec_free(vec: *mut Vec<usize>) {
    let vec: Box<Vec<usize>> = unsafe { Box::from_raw(vec) };

    mem::drop(vec);
}

#[no_mangle]
pub extern "C" fn vec_fetch_data(start: usize, len: usize, buf: *mut usize, vec: *const Vec<usize>) -> usize {
    let array = unsafe { slice::from_raw_parts_mut(buf, len) };
    let vec = unsafe { &*vec };
    let writed = cmp::min(len, vec.len() - start);

    for i in 0..writed {
        array[i] = vec[start + i];
    }

    writed
}
