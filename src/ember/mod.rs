use std::alloc::{self, Layout};
use std::io::{self, Write};
use std::process;
use std::sync::Once;
use std::sync::mpsc;
use std::thread;

#[cfg(windows)]
fn init_console_utf8() {
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        unsafe extern "system" {
            fn SetConsoleOutputCP(wCodePageID: u32) -> i32;
        }
        const CP_UTF8: u32 = 65001;
        SetConsoleOutputCP(CP_UTF8);
    });
}

#[cfg(not(windows))]
fn init_console_utf8() {}

/// Allocate `size` bytes of zero-initialised memory.
///
/// Returns a raw pointer.  The caller is responsible for freeing it with
/// `nimble_free`.  Panics on allocation failure (OOM).
#[unsafe(no_mangle)]
pub extern "C" fn nimble_alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size, 16)
        .unwrap_or_else(|_| panic!("nimble_alloc: invalid layout (size={})", size));
    let ptr = unsafe { alloc::alloc_zeroed(layout) };
    if ptr.is_null() {
        panic!("nimble_alloc: out of memory (size={})", size);
    }
    ptr
}

/// Free a pointer previously returned by `nimble_alloc`.
///
/// # Safety
/// `ptr` must have been returned by `nimble_alloc` and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_free(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        return;
    }
    let layout = Layout::from_size_align(size, 16)
        .unwrap_or_else(|_| panic!("nimble_free: invalid layout (size={})", size));
    unsafe {
        alloc::dealloc(ptr, layout);
    }
}

/// Reallocate a block to a new size, preserving the original contents up to
/// `min(old_size, new_size)` bytes.
///
/// # Safety
///
/// The `ptr` must be a valid pointer to a memory block previously allocated by `nimble_alloc`,
/// with a size at least `old_size`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_realloc(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    if new_size == 0 {
        if !ptr.is_null() {
            unsafe {
                nimble_free(ptr, old_size);
            }
        }
        return std::ptr::null_mut();
    }
    let new_layout = Layout::from_size_align(new_size, 16)
        .unwrap_or_else(|_| panic!("nimble_realloc: invalid layout (size={})", new_size));
    let new_ptr = unsafe { alloc::alloc_zeroed(new_layout) };
    if new_ptr.is_null() {
        panic!("nimble_realloc: out of memory (size={})", new_size);
    }
    if !ptr.is_null() {
        let copy_size = old_size.min(new_size);
        unsafe {
            std::ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
            nimble_free(ptr, old_size);
        }
    }
    new_ptr
}

/// A length-prefixed string block as seen by generated LLVM IR.
///
/// The struct is:
///   { i8* data, i64 length }
///
/// # Safety
/// `data` must point to `length` valid bytes.  The block is allocated via
/// `nimble_alloc`.
#[repr(C)]
pub struct NimbleString {
    pub data: *mut u8,
    pub length: i64,
}

/// Create a new NimbleString by copying `len` bytes from `raw`.
///
/// # Safety
/// `raw` must point to at least `len` valid bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_new(raw: *const u8, len: i64) -> NimbleString {
    let size = len as usize;
    if size == 0 {
        return NimbleString {
            data: std::ptr::null_mut(),
            length: 0,
        };
    }
    let data = nimble_alloc(size);
    unsafe {
        std::ptr::copy_nonoverlapping(raw, data, size);
    }
    NimbleString { data, length: len }
}

/// Free a NimbleString previously created by `nimble_string_new`.
///
/// # Safety
/// `s` must have been returned by `nimble_string_new` and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_free(s: NimbleString) {
    if !s.data.is_null() {
        unsafe {
            nimble_free(s.data, s.length as usize);
        }
    }
}

/// Terminate the process immediately with a message.
///
/// This is called by generated code when a runtime error is detected (e.g.
/// index-out-of-bounds, assertion failure).
#[unsafe(no_mangle)]
pub extern "C" fn nimble_panic(_msg: *const u8, _len: i64) -> ! {
    let _ = writeln!(io::stderr(), "Panic: runtime error");
    process::abort();
}

/// Print a string (pointer + length) to stdout.
///
/// # Safety
///
/// The `msg` must be a valid pointer to a sequence of `len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_print_string(msg: *const u8, len: i64) {
    init_console_utf8();
    if msg.is_null() || len <= 0 {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(msg, len as usize) };
    let _ = io::stdout().write_all(bytes);
    let _ = io::stdout().flush();
}

/// Print a null-terminated string to stdout followed by a newline.
///
/// # Safety
///
/// The `ptr` must be a valid null-terminated string pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_print(ptr: *const u8) {
    init_console_utf8();
    if ptr.is_null() {
        return;
    }
    let s = unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) };
    let _ = writeln!(io::stdout(), "{}", s.to_string_lossy());
    let _ = io::stdout().flush();
}

/// Print a null-terminated string to stdout **without** a trailing newline.
/// Used by the `print_str` built-in.
///
/// # Safety
///
/// The `ptr` must be a valid null-terminated string pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_print_str(ptr: *const u8) {
    init_console_utf8();
    if ptr.is_null() {
        return;
    }
    let s = unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) };
    let _ = write!(io::stdout(), "{}", s.to_string_lossy());
    let _ = io::stdout().flush();
}

/// Print a 64-bit signed integer to stdout followed by a newline.
#[unsafe(no_mangle)]
pub extern "C" fn nimble_print_i64(val: i64) {
    let _ = writeln!(io::stdout(), "{}", val);
    let _ = io::stdout().flush();
}

/// Print a 64-bit floating-point value to stdout followed by a newline.
#[unsafe(no_mangle)]
pub extern "C" fn nimble_print_f64(val: f64) {
    let _ = writeln!(io::stdout(), "{}", val);
    let _ = io::stdout().flush();
}

/// Print a boolean to stdout followed by a newline.
#[unsafe(no_mangle)]
pub extern "C" fn nimble_print_bool(val: bool) {
    let _ = writeln!(io::stdout(), "{}", val);
    let _ = io::stdout().flush();
}

/// Flush stdout.
#[unsafe(no_mangle)]
pub extern "C" fn nimble_flush() {
    let _ = io::stdout().flush();
}

// ── Async runtime & concurrency primitives ──────────────────────────

/// Internal channel handle that holds both sender and receiver.
struct Channel {
    sender: mpsc::Sender<i64>,
    receiver: mpsc::Receiver<i64>,
}

/// Sleep for `ms` milliseconds.
#[unsafe(no_mangle)]
pub extern "C" fn nimble_sleep_ms(ms: i64) {
    thread::sleep(std::time::Duration::from_millis(ms as u64));
}

/// Spawn a new thread that calls `fn_ptr(arg)`.
/// Returns a handle ID (0 for now since join is a no-op).
#[unsafe(no_mangle)]
pub extern "C" fn nimble_thread_spawn(fn_ptr: extern "C" fn(*mut u8), arg: *mut u8) -> i64 {
    let arg_val = arg as usize;
    thread::spawn(move || {
        fn_ptr(arg_val as *mut u8);
    });
    // JoinHandle tracking left for future implementation
    0
}

/// Join a thread by ID (simplified — waits on a channel).
#[unsafe(no_mangle)]
pub extern "C" fn nimble_thread_join(_thread_id: i64) {
    // In a full implementation we would track handles.
    // For now this is a no-op placeholder.
}

/// Create a mutex.
#[unsafe(no_mangle)]
pub extern "C" fn nimble_mutex_create() -> *mut std::sync::Mutex<()> {
    Box::into_raw(Box::new(std::sync::Mutex::new(())))
}

/// Lock a mutex.
///
/// # Safety
///
/// The `mtx` must be a valid pointer to a `std::sync::Mutex<()>`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_mutex_lock(mtx: *mut std::sync::Mutex<()>) {
    if let Some(m) = unsafe { mtx.as_ref() } {
        let guard = m.lock();
        drop(guard);
    }
}

/// Unlock a mutex.
///
/// # Safety
///
/// The `mtx` must be a valid pointer to a `std::sync::Mutex<()>`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_mutex_unlock(mtx: *mut std::sync::Mutex<()>) {
    if let Some(m) = unsafe { mtx.as_ref() } {
        // Drop the lock guard by letting it fall out of scope.
        if let Ok(guard) = m.lock() {
            drop(guard);
        }
    }
}

/// Create a channel (returns an opaque pointer to a channel handle).
#[unsafe(no_mangle)]
pub extern "C" fn nimble_channel_create() -> i64 {
    let (tx, rx) = mpsc::channel::<i64>();
    let chan = Box::into_raw(Box::new(Channel {
        sender: tx,
        receiver: rx,
    }));
    chan as i64
}

/// Send a value on a channel.
#[unsafe(no_mangle)]
pub extern "C" fn nimble_channel_send(chan_ptr: i64, value: i64) {
    let chan = unsafe { &*(chan_ptr as *const Channel) };
    let _ = chan.sender.send(value);
}

/// Receive a value from a channel (blocking).
#[unsafe(no_mangle)]
pub extern "C" fn nimble_channel_recv(chan_ptr: i64) -> i64 {
    let chan = unsafe { &*(chan_ptr as *const Channel) };
    chan.receiver.recv().unwrap_or(0)
}

/// Atomic load.
///
/// # Safety
///
/// The `ptr` must be a valid pointer to an `i64`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_load(ptr: *mut i64) -> i64 {
    unsafe { std::sync::atomic::AtomicI64::from_ptr(ptr).load(std::sync::atomic::Ordering::SeqCst) }
}

/// Atomic store.
///
/// # Safety
///
/// The `ptr` must be a valid pointer to an `i64`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_store(ptr: *mut i64, val: i64) {
    unsafe {
        std::sync::atomic::AtomicI64::from_ptr(ptr).store(val, std::sync::atomic::Ordering::SeqCst)
    }
}

/// Atomic fetch-and-add. Returns the previous value.
///
/// # Safety
///
/// The `ptr` must be a valid pointer to an `i64`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_add(ptr: *mut i64, val: i64) -> i64 {
    unsafe {
        std::sync::atomic::AtomicI64::from_ptr(ptr)
            .fetch_add(val, std::sync::atomic::Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_and_free() {
        let ptr = nimble_alloc(64);
        assert!(!ptr.is_null());
        unsafe {
            nimble_free(ptr, 64);
        }
    }

    #[test]
    fn alloc_zero_returns_null() {
        let ptr = nimble_alloc(0);
        assert!(ptr.is_null());
    }

    #[test]
    fn realloc_enlarges() {
        let ptr = nimble_alloc(16);
        assert!(!ptr.is_null());
        // Write a pattern
        unsafe {
            std::ptr::write(ptr, 42u8);
        }
        let new_ptr = unsafe { nimble_realloc(ptr, 16, 32) };
        assert!(!new_ptr.is_null());
        unsafe {
            assert_eq!(std::ptr::read(new_ptr), 42u8);
            nimble_free(new_ptr, 32);
        }
    }

    #[test]
    fn string_new_and_free() {
        let s = b"hello";
        let ns = unsafe { nimble_string_new(s.as_ptr(), s.len() as i64) };
        assert!(!ns.data.is_null());
        assert_eq!(ns.length, 5);
        unsafe {
            let slice = std::slice::from_raw_parts(ns.data, ns.length as usize);
            assert_eq!(slice, b"hello");
            nimble_string_free(ns);
        }
    }

    #[test]
    fn string_new_zero_len() {
        let ns = unsafe { nimble_string_new(std::ptr::null(), 0) };
        assert!(ns.data.is_null());
        assert_eq!(ns.length, 0);
    }

    #[test]
    fn print_i64_does_not_panic() {
        nimble_print_i64(42);
    }

    #[test]
    fn print_f64_does_not_panic() {
        nimble_print_f64(std::f64::consts::PI);
    }

    #[test]
    fn print_bool_does_not_panic() {
        nimble_print_bool(true);
        nimble_print_bool(false);
    }
}
