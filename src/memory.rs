// copy from https://github.com/rustwasm/wasm-bindgen/blob/17874c4cfc3a6c3e426b0bfc2bd17d99f1fb2cf3/src/lib.rs#L1529-L1586
use std::alloc::{alloc, Layout};
use std::mem;

#[no_mangle]
pub extern "C" fn __wasm_malloc(size: usize) -> *mut u8 {
    let align = mem::align_of::<usize>();
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                let ptr = alloc(layout);
                if !ptr.is_null() {
                    return ptr;
                }
            } else {
                return align as *mut u8;
            }
        }
    }

    panic!("invalid malloc request")
}
