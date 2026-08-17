//! Week 10 — FFI, unsafe, systems programming.
//!
//! Every `unsafe` block here is small, justified, and confined — the
//! safe-wrapper pattern. Deliberately-crashing examples (e.g. dereferencing a
//! null pointer) are NOT run as tests because they abort the process; they
//! stay as documented exercises in the course.

/// Raw pointers: creating one is safe; dereferencing it is `unsafe`.
pub fn raw_pointer_write() -> i32 {
    let mut x = 42;
    let ptr: *mut i32 = &mut x; // safe to create
    unsafe {
        *ptr = 100; // unsafe to dereference
    }
    x
}

/// Raw pointer arithmetic on a Vec's buffer (`ptr.add(n)`).
/// (The course says "allocate a Vec<i32>" — kept as a `Vec` on purpose.)
#[allow(clippy::useless_vec)]
pub fn raw_pointer_arithmetic() -> i32 {
    let v = vec![10, 20, 30];
    let ptr = v.as_ptr();
    unsafe { *ptr.add(1) }
}

extern "C" {
    fn getpid() -> i32; // from libc (linked by default on Unix)
    fn strlen(s: *const u8) -> usize;
}

/// Calling a foreign function is `unsafe`.
pub fn ffi_getpid() -> i32 {
    unsafe { getpid() }
}

/// `s` must be NUL-terminated — the caller guarantees it.
pub fn ffi_strlen(s: &[u8]) -> usize {
    unsafe { strlen(s.as_ptr()) }
}

/// `CString` is owned and NUL-terminated; it rejects embedded NUL bytes.
pub fn cstring_new() -> Result<(), std::ffi::NulError> {
    let _c = std::ffi::CString::new("hello")?;
    Ok(())
}

pub fn cstring_rejects_nul() -> bool {
    std::ffi::CString::new("he\0llo").is_err()
}

/// The safe-wrapper pattern: a safe function that contains all the `unsafe`.
/// Returns the process id.
pub fn current_pid() -> u32 {
    unsafe { libc::getpid() as u32 }
}

/// The cdylib example's `add` — `#[no_mangle]` + `extern "C"` so C can link
/// to it by its plain name. Testable as an ordinary function.
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_pointer_write_changes_value() {
        assert_eq!(raw_pointer_write(), 100);
    }

    #[test]
    fn raw_pointer_arithmetic_reads_element() {
        assert_eq!(raw_pointer_arithmetic(), 20);
    }

    #[test]
    fn ffi_getpid_returns_a_pid() {
        assert!(ffi_getpid() > 0);
    }

    #[test]
    fn ffi_strlen_counts_bytes() {
        let s = b"hello\0";
        assert_eq!(ffi_strlen(s), 5);
    }

    #[test]
    fn cstring_handles_nul() {
        cstring_new().unwrap();
        assert!(cstring_rejects_nul());
    }

    #[test]
    fn safe_wrapper_works() {
        assert!(current_pid() > 0);
    }

    #[test]
    fn extern_c_add_works() {
        assert_eq!(add(2, 3), 5);
    }
}
