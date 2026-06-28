use std::alloc::{self, Layout};
use std::collections::HashMap;
use std::ffi::CStr;
use std::io::{self, BufRead, Read, Write};
use std::process;
use std::sync::Once;
use std::sync::mpsc;
use std::thread;
use std::time;

// ── Platform init ────────────────────────────────────────────────────

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

// ── Memory ───────────────────────────────────────────────────────────

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_free(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        return;
    }
    let layout = Layout::from_size_align(size, 16)
        .unwrap_or_else(|_| panic!("nimble_free: invalid layout (size={})", size));
    unsafe { alloc::dealloc(ptr, layout) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_realloc(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    if new_size == 0 {
        if !ptr.is_null() {
            unsafe { nimble_free(ptr, old_size) }
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

// ── String type (length-prefixed) ────────────────────────────────────

#[repr(C)]
pub struct NimbleString {
    pub data: *mut u8,
    pub length: i64,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_new(raw: *const u8, len: i64) -> NimbleString {
    if len <= 0 {
        return NimbleString { data: std::ptr::null_mut(), length: 0 };
    }
    let size = len as usize;
    let data = nimble_alloc(size + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(raw, data, size);
        *data.add(size) = 0;
    }
    NimbleString { data, length: len }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_free(s: NimbleString) {
    if !s.data.is_null() {
        unsafe { nimble_free(s.data, (s.length + 1) as usize) }
    }
}

unsafe fn to_str(ptr: *const u8, len: i64) -> &'static str {
    if ptr.is_null() || len <= 0 {
        return "";
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    std::str::from_utf8(slice).unwrap_or("")
}

unsafe fn to_cstr(ptr: *const u8) -> &'static str {
    if ptr.is_null() {
        return "";
    }
    unsafe { CStr::from_ptr(ptr as *const i8) }.to_str().unwrap_or("")
}

// ── String operations ────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_length(s: *const u8) -> i64 {
    to_cstr(s).len() as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_concat(a: *const u8, b: *const u8) -> *mut u8 {
    let sa = to_cstr(a);
    let sb = to_cstr(b);
    let result = format!("{}{}", sa, sb);
    let bytes = result.as_bytes();
    let len = bytes.len();
    let ptr = nimble_alloc(len + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
        *ptr.add(len) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_eq(a: *const u8, b: *const u8) -> i64 {
    let sa = to_cstr(a);
    let sb = to_cstr(b);
    if sa == sb { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_substring(s: *const u8, start: i64, end: i64) -> *mut u8 {
    let src = to_cstr(s);
    let len = src.len() as i64;
    let s = start.max(0).min(len);
    let e = end.max(s).min(len);
    let sub = &src[s as usize..e as usize];
    let bytes = sub.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_find(s: *const u8, needle: *const u8) -> i64 {
    let src = to_cstr(s);
    let n = to_cstr(needle);
    src.find(n).map(|i| i as i64).unwrap_or(-1)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_trim(s: *const u8) -> *mut u8 {
    let src = to_cstr(s);
    let trimmed = src.trim();
    let bytes = trimmed.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_to_upper(s: *const u8) -> *mut u8 {
    let src = to_cstr(s);
    let result: String = src.to_uppercase();
    let bytes = result.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_to_lower(s: *const u8) -> *mut u8 {
    let src = to_cstr(s);
    let result: String = src.to_lowercase();
    let bytes = result.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_starts_with(s: *const u8, prefix: *const u8) -> i64 {
    if to_cstr(s).starts_with(to_cstr(prefix)) { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_ends_with(s: *const u8, suffix: *const u8) -> i64 {
    if to_cstr(s).ends_with(to_cstr(suffix)) { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_replace(s: *const u8, from: *const u8, to: *const u8) -> *mut u8 {
    let src = to_cstr(s);
    let result = src.replace(to_cstr(from), to_cstr(to));
    let bytes = result.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_repeat(s: *const u8, count: i64) -> *mut u8 {
    let src = to_cstr(s);
    let result = src.repeat(count.max(0) as usize);
    let bytes = result.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_split(s: *const u8, delim: i64) -> i64 {
    let src = to_cstr(s);
    let d = char::from_u32(delim as u32).unwrap_or(',');
    let count = src.split(d).count() as i64;
    count
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_split_nth(s: *const u8, delim: i64, n: i64) -> *mut u8 {
    let src = to_cstr(s);
    let d = char::from_u32(delim as u32).unwrap_or(',');
    let parts: Vec<&str> = src.split(d).collect();
    if n >= 0 && (n as usize) < parts.len() {
        let part = parts[n as usize];
        let bytes = part.as_bytes();
        let ptr = nimble_alloc(bytes.len() + 1);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            *ptr.add(bytes.len()) = 0;
        }
        ptr
    } else {
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_int_to_string(val: i64) -> *mut u8 {
    let s = format!("{}", val);
    let bytes = s.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_float_to_string(val: f64) -> *mut u8 {
    let s = format!("{}", val);
    let bytes = s.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_to_int(s: *const u8) -> i64 {
    to_cstr(s).trim().parse::<i64>().unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_string_to_float(s: *const u8) -> f64 {
    to_cstr(s).trim().parse::<f64>().unwrap_or(0.0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_bool_to_string(val: i64) -> *mut u8 {
    let s = if val != 0 { "true" } else { "false" };
    let bytes = s.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

// ── I/O ──────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_print_string(msg: *const u8, len: i64) {
    init_console_utf8();
    if msg.is_null() || len <= 0 { return; }
    let bytes = unsafe { std::slice::from_raw_parts(msg, len as usize) };
    let _ = io::stdout().write_all(bytes);
    let _ = io::stdout().flush();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_print(ptr: *const u8) {
    init_console_utf8();
    if ptr.is_null() { return; }
    let s = unsafe { CStr::from_ptr(ptr as *const i8) };
    let _ = writeln!(io::stdout(), "{}", s.to_string_lossy());
    let _ = io::stdout().flush();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_print_str(ptr: *const u8) {
    init_console_utf8();
    if ptr.is_null() { return; }
    let s = unsafe { CStr::from_ptr(ptr as *const i8) };
    let _ = write!(io::stdout(), "{}", s.to_string_lossy());
    let _ = io::stdout().flush();
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_print_i64(val: i64) {
    let _ = writeln!(io::stdout(), "{}", val);
    let _ = io::stdout().flush();
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_print_f64(val: f64) {
    let _ = writeln!(io::stdout(), "{}", val);
    let _ = io::stdout().flush();
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_print_bool(val: bool) {
    let _ = writeln!(io::stdout(), "{}", val);
    let _ = io::stdout().flush();
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_flush() {
    let _ = io::stdout().flush();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_read_line() -> *mut u8 {
    init_console_utf8();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim_end_matches('\n').trim_end_matches('\r');
            let bytes = trimmed.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            *ptr.add(bytes.len()) = 0;
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_read_file(path: *const u8) -> *mut u8 {
    let p = to_cstr(path);
    match std::fs::read_to_string(p) {
        Ok(content) => {
            let bytes = content.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            *ptr.add(bytes.len()) = 0;
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_write_file(path: *const u8, content: *const u8) -> i64 {
    let p = to_cstr(path);
    let c = to_cstr(content);
    match std::fs::write(p, c) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_append_file(path: *const u8, content: *const u8) -> i64 {
    let p = to_cstr(path);
    let c = to_cstr(content);
    match std::fs::OpenOptions::new().append(true).create(true).open(p) {
        Ok(mut file) => {
            match file.write_all(c.as_bytes()) {
                Ok(_) => 0,
                Err(_) => -2,
            }
        }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_file_exists(path: *const u8) -> i64 {
    let p = to_cstr(path);
    if std::path::Path::new(p).exists() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_file_size(path: *const u8) -> i64 {
    let p = to_cstr(path);
    std::fs::metadata(p).map(|m| m.len() as i64).unwrap_or(-1)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_delete_file(path: *const u8) -> i64 {
    let p = to_cstr(path);
    match std::fs::remove_file(p) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_rename_file(old: *const u8, new: *const u8) -> i64 {
    let o = to_cstr(old);
    let n = to_cstr(new);
    match std::fs::rename(o, n) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_create_dir(path: *const u8) -> i64 {
    let p = to_cstr(path);
    match std::fs::create_dir_all(p) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_remove_dir(path: *const u8) -> i64 {
    let p = to_cstr(path);
    match std::fs::remove_dir_all(p) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_list_dir(path: *const u8) -> *mut u8 {
    let p = to_cstr(path);
    match std::fs::read_dir(p) {
        Ok(entries) => {
            let mut result = String::new();
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    result.push_str(&name);
                    result.push('\n');
                }
            }
            let bytes = result.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
                *ptr.add(bytes.len()) = 0;
            }
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_current_dir() -> *mut u8 {
    match std::env::current_dir() {
        Ok(p) => {
            let s = p.to_string_lossy().to_string();
            let bytes = s.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
                *ptr.add(bytes.len()) = 0;
            }
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

// ── Time ─────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn nimble_clock_monotonic() -> f64 {
    time::Instant::now()
        .duration_since(time::Instant::now())
        .as_secs_f64()
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_time_nanos() -> i64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_time_seconds() -> i64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_time_format(fmt: *const u8) -> *mut u8 {
    let format_str = to_cstr(fmt);
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let result = format!("{}", now);
    let bytes = result.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_sleep_ms(ms: i64) {
    thread::sleep(time::Duration::from_millis(ms as u64));
}

// ── Math (extended) ──────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn nimble_sin(x: f64) -> f64 { x.sin() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_cos(x: f64) -> f64 { x.cos() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_tan(x: f64) -> f64 { x.tan() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_asin(x: f64) -> f64 { x.asin() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_acos(x: f64) -> f64 { x.acos() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_atan(x: f64) -> f64 { x.atan() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_atan2(y: f64, x: f64) -> f64 { y.atan2(x) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_sinh(x: f64) -> f64 { x.sinh() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_cosh(x: f64) -> f64 { x.cosh() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_tanh(x: f64) -> f64 { x.tanh() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_sqrt(x: f64) -> f64 { x.sqrt() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_cbrt(x: f64) -> f64 { x.cbrt() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_pow(b: f64, e: f64) -> f64 { b.powf(e) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_floor(x: f64) -> f64 { x.floor() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_ceil(x: f64) -> f64 { x.ceil() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_round(x: f64) -> f64 { x.round() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_trunc(x: f64) -> f64 { x.trunc() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_fabs(x: f64) -> f64 { x.abs() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_log(x: f64) -> f64 { x.ln() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_log2(x: f64) -> f64 { x.log2() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_log10(x: f64) -> f64 { x.log10() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_exp(x: f64) -> f64 { x.exp() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_fma(a: f64, b: f64, c: f64) -> f64 { a.mul_add(b, c) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_hypot(a: f64, b: f64) -> f64 { a.hypot(b) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_fmod(a: f64, b: f64) -> f64 { a % b }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_is_nan(x: f64) -> i64 { if x.is_nan() { 1 } else { 0 } }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_is_infinite(x: f64) -> i64 { if x.is_infinite() { 1 } else { 0 } }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_is_finite(x: f64) -> i64 { if x.is_finite() { 1 } else { 0 } }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_next_after(x: f64, y: f64) -> f64 {
    if x < y { (x + (y - x) * 0.001).min(y) }
    else if x > y { (x - (x - y) * 0.001).max(y) }
    else { x }
}

// Integer math
#[unsafe(no_mangle)]
pub extern "C" fn nimble_abs_i64(x: i64) -> i64 { x.abs() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_min_i64(a: i64, b: i64) -> i64 { a.min(b) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_max_i64(a: i64, b: i64) -> i64 { a.max(b) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_clamp_i64(v: i64, lo: i64, hi: i64) -> i64 { v.clamp(lo, hi) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_popcount(x: i64) -> i64 { x.count_ones() as i64 }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_clz(x: i64) -> i64 { x.leading_zeros() as i64 }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_ctz(x: i64) -> i64 { x.trailing_zeros() as i64 }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_byte_swap(x: i64) -> i64 { x.swap_bytes() }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_rotl(x: i64, n: i64) -> i64 { x.rotate_left(n as u32) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_rotr(x: i64, n: i64) -> i64 { x.rotate_right(n as u32) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_checked_add(a: i64, b: i64) -> i64 {
    match a.checked_add(b) { Some(v) => v, None => panic!("integer overflow") }
}
#[unsafe(no_mangle)]
pub extern "C" fn nimble_checked_sub(a: i64, b: i64) -> i64 {
    match a.checked_sub(b) { Some(v) => v, None => panic!("integer underflow") }
}
#[unsafe(no_mangle)]
pub extern "C" fn nimble_checked_mul(a: i64, b: i64) -> i64 {
    match a.checked_mul(b) { Some(v) => v, None => panic!("integer overflow") }
}
#[unsafe(no_mangle)]
pub extern "C" fn nimble_checked_div(a: i64, b: i64) -> i64 {
    match a.checked_div(b) { Some(v) => v, None => panic!("division by zero") }
}
#[unsafe(no_mangle)]
pub extern "C" fn nimble_saturating_add(a: i64, b: i64) -> i64 { a.saturating_add(b) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_saturating_sub(a: i64, b: i64) -> i64 { a.saturating_sub(b) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_wrapping_add(a: i64, b: i64) -> i64 { a.wrapping_add(b) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_wrapping_sub(a: i64, b: i64) -> i64 { a.wrapping_sub(b) }
#[unsafe(no_mangle)]
pub extern "C" fn nimble_wrapping_mul(a: i64, b: i64) -> i64 { a.wrapping_mul(b) }

// ── Random numbers ──────────────────────────────────────────────────

use std::time::{SystemTime, UNIX_EPOCH};

static mut RNG_STATE: u64 = 0;

fn xoshiro256ss_next() -> u64 {
    unsafe {
        if RNG_STATE == 0 {
            RNG_STATE = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;
        }
        let result = RNG_STATE.wrapping_mul(0x9E3779B97F4A7C15);
        RNG_STATE = RNG_STATE.rotate_left(23) ^ RNG_STATE.wrapping_shr(18);
        RNG_STATE = RNG_STATE ^ RNG_STATE.wrapping_shr(5);
        RNG_STATE = RNG_STATE.wrapping_mul(0x9E3779B97F4A7C15);
        result
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_rand_i64() -> i64 {
    xoshiro256ss_next() as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_rand_f64() -> f64 {
    (xoshiro256ss_next() >> 11) as f64 * (1.0 / 9007199254740992.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_rand_range(lo: i64, hi: i64) -> i64 {
    if lo >= hi { return lo; }
    let range = (hi - lo).unsigned_abs();
    let val = xoshiro256ss_next() % range;
    lo + val as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_srand(seed: u64) {
    unsafe { RNG_STATE = seed; }
}

// ── Hashing ──────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_hash_fnv1a(data: *const u8, len: i64) -> i64 {
    let bytes = unsafe { std::slice::from_raw_parts(data, len.max(0) as usize) };
    let mut hash: u64 = 0xCBF29CE484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001B3);
    }
    hash as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_hash_sip(data: *const u8, len: i64) -> i64 {
    let bytes = unsafe { std::slice::from_raw_parts(data, len.max(0) as usize) };
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write(bytes);
    hasher.finish() as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_hash_xxhash3(data: *const u8, len: i64) -> i64 {
    let bytes = unsafe { std::slice::from_raw_parts(data, len.max(0) as usize) };
    let mut hash: u64 = 0;
    for chunk in bytes.chunks(16) {
        let mut acc = 0u64;
        for &b in chunk {
            acc = acc.wrapping_mul(0x9E3779B97F4A7C15);
            acc ^= b as u64;
        }
        hash = hash.wrapping_add(acc);
    }
    hash as i64
}

// ── Base64 encoding ─────────────────────────────────────────────────

const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64_URL_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_base64_encode(data: *const u8, len: i64, url_safe: i64) -> *mut u8 {
    let bytes = unsafe { std::slice::from_raw_parts(data, len.max(0) as usize) };
    let chars = if url_safe != 0 { BASE64_URL_CHARS } else { BASE64_CHARS };
    let encoded = base64_encode_slice(bytes, chars);
    let ptr = nimble_alloc(encoded.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(encoded.as_ptr(), ptr, encoded.len());
        *ptr.add(encoded.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_base64_decode(data: *const u8, len: i64) -> *mut u8 {
    let s = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(data, len.max(0) as usize)) };
    match base64_decode_to_string(s) {
        Some(decoded) => {
            let ptr = nimble_alloc(decoded.len() + 1);
            unsafe {
                std::ptr::copy_nonoverlapping(decoded.as_ptr(), ptr, decoded.len());
                *ptr.add(decoded.len()) = 0;
            }
            ptr
        }
        None => std::ptr::null_mut(),
    }
}

fn base64_encode_slice(data: &[u8], chars: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(((data.len() + 2) / 3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(chars[((triple >> 18) & 0x3F) as usize]);
        result.push(chars[((triple >> 12) & 0x3F) as usize]);
        result.push(if chunk.len() > 1 { chars[((triple >> 6) & 0x3F) as usize] } else { b'=' });
        result.push(if chunk.len() > 2 { chars[(triple & 0x3F) as usize] } else { b'=' });
    }
    result
}

fn base64_decode_to_string(s: &str) -> Option<Vec<u8>> {
    let s = s.trim_end_matches('=');
    let mut result = Vec::with_capacity((s.len() * 3) / 4);
    for chunk in s.as_bytes().chunks(4) {
        let mut quad = 0u32;
        for (i, &c) in chunk.iter().enumerate() {
            let val = match c {
                b'A'..=b'Z' => c - b'A',
                b'a'..=b'z' => c - b'a' + 26,
                b'0'..=b'9' => c - b'0' + 52,
                b'+' | b'-' => 62,
                b'/' | b'_' => 63,
                _ => return None,
            } as u32;
            quad |= val << (6 * (3 - i));
        }
        result.push(((quad >> 16) & 0xFF) as u8);
        if chunk.len() > 2 { result.push(((quad >> 8) & 0xFF) as u8); }
        if chunk.len() > 3 { result.push((quad & 0xFF) as u8); }
    }
    Some(result)
}

// ── Hex encoding ────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_hex_encode(data: *const u8, len: i64, upper: i64) -> *mut u8 {
    let bytes = unsafe { std::slice::from_raw_parts(data, len.max(0) as usize) };
    let hex: String = if upper != 0 {
        bytes.iter().map(|b| format!("{:02X}", b)).collect()
    } else {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    };
    let ptr = nimble_alloc(hex.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(hex.as_ptr(), ptr, hex.len());
        *ptr.add(hex.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_hex_decode(hex: *const u8, len: i64) -> *mut u8 {
    let s = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(hex, len.max(0) as usize)) };
    let s = s.trim();
    if s.len() % 2 != 0 { return std::ptr::null_mut(); }
    let mut bytes = Vec::with_capacity(s.len() / 2);
    for chunk in s.as_bytes().chunks(2) {
        let hi = match (chunk[0] as char).to_digit(16) {
            Some(d) => d as u8,
            None => return std::ptr::null_mut(),
        };
        let lo = match (chunk[1] as char).to_digit(16) {
            Some(d) => d as u8,
            None => return std::ptr::null_mut(),
        };
        bytes.push((hi << 4) | lo);
    }
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

// ── UTF-8 validation ────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_utf8_validate(data: *const u8, len: i64) -> i64 {
    let bytes = unsafe { std::slice::from_raw_parts(data, len.max(0) as usize) };
    if std::str::from_utf8(bytes).is_ok() { 1 } else { 0 }
}

// ── JSON ─────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_json_parse(json: *const u8) -> *mut u8 {
    let s = to_cstr(json);
    match json_parse_to_string(s) {
        Some(parsed) => {
            let bytes = parsed.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            *ptr.add(bytes.len()) = 0;
            ptr
        }
        None => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_json_stringify(value: *const u8) -> *mut u8 {
    let s = to_cstr(value);
    let bytes = s.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_json_get_field(json: *const u8, field: *const u8) -> *mut u8 {
    let s = to_cstr(json);
    let f = to_cstr(field);
    let key = format!("\"{}\"", f);
    if let Some(pos) = s.find(&key) {
        let rest = &s[pos + key.len()..];
        let trimmed = rest.trim_start();
        if trimmed.starts_with(':') {
            let value = trimmed[1..].trim_start();
            let end = find_json_value_end(value);
            let val = &value[..end];
            let bytes = val.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
                *ptr.add(bytes.len()) = 0;
            }
            return ptr;
        }
    }
    std::ptr::null_mut()
}

fn find_json_value_end(s: &str) -> usize {
    let s = s.trim_start();
    if s.is_empty() { return 0; }
    if s.starts_with('"') {
        let mut chars = s[1..].char_indices();
        while let Some((i, c)) = chars.next() {
            if c == '\\' { chars.next(); continue; }
            if c == '"' { return i + 2; }
        }
        return s.len();
    }
    if s.starts_with('{') || s.starts_with('[') {
        let close = if s.starts_with('{') { '}' } else { ']' };
        let mut depth = 0;
        for (i, c) in s.char_indices() {
            if c == '{' || c == '[' { depth += 1; }
            else if c == '}' || c == ']' {
                depth -= 1;
                if depth == 0 { return i + 1; }
            }
        }
        return s.len();
    }
    s.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(s.len())
}

fn json_parse_to_string(s: &str) -> Option<String> {
    Some(format!("parsed: {}", s.len()))
}

// ── CLI / Environment ────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_getenv(name: *const u8) -> *mut u8 {
    let n = to_cstr(name);
    match std::env::var(n) {
        Ok(val) => {
            let bytes = val.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            *ptr.add(bytes.len()) = 0;
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_setenv(name: *const u8, value: *const u8) -> i64 {
    #[cfg(windows)] {
        unsafe extern "system" {
            fn SetEnvironmentVariableA(lpName: *const u8, lpValue: *const u8) -> i32;
        }
        unsafe { SetEnvironmentVariableA(name, value); }
    }
    #[cfg(not(windows))] {
        unsafe extern "C" {
            fn setenv(name: *const u8, value: *const u8, overwrite: i32) -> i32;
        }
        unsafe { setenv(name, value, 1); }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_args_count() -> i64 {
    std::env::args().len() as i64 - 1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_args_get(index: i64) -> *mut u8 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if index >= 0 && (index as usize) < args.len() {
        let s = &args[index as usize];
        let bytes = s.as_bytes();
        let ptr = nimble_alloc(bytes.len() + 1);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
        ptr
    } else {
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_exit(code: i64) {
    process::exit(code as i32);
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_abort() {
    process::abort();
}

// ── Terminal ─────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn nimble_is_terminal() -> i64 {
    #[cfg(windows)] {
        unsafe extern "system" {
            fn GetStdHandle(nStdHandle: u32) -> isize;
            fn GetConsoleMode(hConsoleHandle: isize, lpMode: *mut u32) -> i32;
        }
        let handle = unsafe { GetStdHandle(0xFFFFFFF5) };
        if handle == -1 || handle == 0 { return 0; }
        let mut mode: u32 = 0;
        if unsafe { GetConsoleMode(handle, &mut mode) } != 0 { 1 } else { 0 }
    }
    #[cfg(not(windows))] {
        if unsafe { libc::isatty(1) } != 0 { 1 } else { 0 }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_terminal_width() -> i64 {
    #[cfg(windows)] {
        80
    }
    #[cfg(not(windows))] {
        80
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_terminal_height() -> i64 {
    #[cfg(windows)] {
        24
    }
    #[cfg(not(windows))] {
        24
    }
}

// ── System info ──────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_hostname() -> *mut u8 {
    #[cfg(windows)] {
        let mut buffer = [0u8; 256];
        unsafe extern "system" {
            fn GetComputerNameA(lpBuffer: *mut u8, nSize: *mut u32) -> i32;
        }
        let mut size: u32 = 256;
        if unsafe { GetComputerNameA(buffer.as_mut_ptr(), &mut size) } != 0 {
            let len = size as usize;
            let ptr = nimble_alloc(len + 1);
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), ptr, len);
            *ptr.add(len) = 0;
            return ptr;
        }
    }
    #[cfg(not(windows))] {
        let hostname = std::process::Command::new("hostname")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok());
        if let Some(h) = hostname {
            let trimmed = h.trim().to_string();
            let bytes = trimmed.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            *ptr.add(bytes.len()) = 0;
            return ptr;
        }
    }
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_os_name() -> *mut u8 {
    let s = if cfg!(windows) { "windows" } else if cfg!(target_os = "macos") { "macos" } else { "linux" };
    let bytes = s.as_bytes();
    let ptr = nimble_alloc(bytes.len() + 1);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
    *ptr.add(bytes.len()) = 0;
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_cpu_count() -> i64 {
    std::thread::available_parallelism().map(|n| n.get() as i64).unwrap_or(1)
}

// ── Panic ────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn nimble_panic(msg: *const u8) {
    let s = unsafe { to_cstr(msg) };
    let _ = writeln!(io::stderr(), "Panic: {}", s);
    process::abort();
}

// ── Concurrency (extended) ───────────────────────────────────────────

struct Channel {
    sender: mpsc::Sender<i64>,
    receiver: mpsc::Receiver<i64>,
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_channel_create() -> i64 {
    let (tx, rx) = mpsc::channel::<i64>();
    let chan = Box::into_raw(Box::new(Channel { sender: tx, receiver: rx }));
    chan as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_channel_send(chan_ptr: i64, value: i64) {
    let chan = unsafe { &*(chan_ptr as *const Channel) };
    let _ = chan.sender.send(value);
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_channel_recv(chan_ptr: i64) -> i64 {
    let chan = unsafe { &*(chan_ptr as *const Channel) };
    chan.receiver.recv().unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_mutex_create() -> *mut std::sync::Mutex<()> {
    Box::into_raw(Box::new(std::sync::Mutex::new(())))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_mutex_lock(mtx: *mut std::sync::Mutex<()>) {
    if let Some(m) = unsafe { mtx.as_ref() } {
        let guard = m.lock();
        drop(guard);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_mutex_unlock(mtx: *mut std::sync::Mutex<()>) {
    if let Some(m) = mtx.as_ref() {
        if let Ok(guard) = m.lock() { drop(guard); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_thread_spawn(fn_ptr: extern "C" fn(*mut u8), arg: *mut u8) -> i64 {
    let arg_ptr = arg as usize;
    thread::spawn(move || {
        let fn_ptr: extern "C" fn(*mut u8) = fn_ptr;
        fn_ptr(arg_ptr as *mut u8);
    });
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_thread_join(_thread_id: i64) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_load(ptr: *mut i64) -> i64 {
    unsafe { std::sync::atomic::AtomicI64::from_ptr(ptr).load(std::sync::atomic::Ordering::SeqCst) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_store(ptr: *mut i64, val: i64) {
    unsafe { std::sync::atomic::AtomicI64::from_ptr(ptr).store(val, std::sync::atomic::Ordering::SeqCst) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_add(ptr: *mut i64, val: i64) -> i64 {
    unsafe { std::sync::atomic::AtomicI64::from_ptr(ptr).fetch_add(val, std::sync::atomic::Ordering::SeqCst) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_sub(ptr: *mut i64, val: i64) -> i64 {
    unsafe { std::sync::atomic::AtomicI64::from_ptr(ptr).fetch_sub(val, std::sync::atomic::Ordering::SeqCst) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_swap(ptr: *mut i64, val: i64) -> i64 {
    unsafe { std::sync::atomic::AtomicI64::from_ptr(ptr).swap(val, std::sync::atomic::Ordering::SeqCst) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_atomic_cas(ptr: *mut i64, old: i64, new: i64) -> i64 {
    unsafe { std::sync::atomic::AtomicI64::from_ptr(ptr).compare_exchange(old, new, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).unwrap_or(old) }
}

// ── Sort (runtime helper) ───────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_sort_i64(data: *mut i64, len: i64) {
    let slice = unsafe { std::slice::from_raw_parts_mut(data, len.max(0) as usize) };
    slice.sort();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_sort_f64(data: *mut f64, len: i64) {
    let slice = unsafe { std::slice::from_raw_parts_mut(data, len.max(0) as usize) };
    slice.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
}

// ── Networking ────────────────────────────────────────────────────────

use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Mutex as StdMutex;
use std::sync::LazyLock;

static TCP_CONNECTIONS: LazyLock<StdMutex<HashMap<i64, TcpStream>>> = LazyLock::new(|| StdMutex::new(HashMap::new()));
static NEXT_TCP_ID: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(1);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_net_connect(host: *const u8, port: i64) -> i64 {
    let h = unsafe { to_cstr(host) };
    let addr_str = format!("{}:{}", h, port);
    match TcpStream::connect(&addr_str) {
        Ok(stream) => {
            let id = NEXT_TCP_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if let Ok(mut map) = TCP_CONNECTIONS.lock() {
                map.insert(id, stream);
            }
            id
        }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_net_send(fd: i64, data: *const u8, len: i64) -> i64 {
    let bytes = unsafe { std::slice::from_raw_parts(data, len.max(0) as usize) };
    if let Ok(mut map) = TCP_CONNECTIONS.lock() {
        if let Some(stream) = map.get_mut(&fd) {
            match stream.write(bytes) { Ok(n) => return n as i64, Err(_) => return -1 }
        }
    }
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_net_recv(fd: i64, buffer: *mut u8, size: i64) -> i64 {
    let buf = unsafe { std::slice::from_raw_parts_mut(buffer, size.max(0) as usize) };
    if let Ok(mut map) = TCP_CONNECTIONS.lock() {
        if let Some(stream) = map.get_mut(&fd) {
            let _ = stream.set_read_timeout(Some(time::Duration::from_secs(5)));
            match stream.read(buf) { Ok(n) => return n as i64, Err(_) => return -1 }
        }
    }
    -1
}

#[unsafe(no_mangle)]
pub extern "C" fn nimble_net_close(fd: i64) -> i64 {
    if let Ok(mut map) = TCP_CONNECTIONS.lock() {
        map.remove(&fd);
        0
    } else {
        -1
    }
}

// ── DNS ──────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nimble_dns_resolve(host: *const u8) -> *mut u8 {
    let h = unsafe { to_cstr(host) };
    match (h, 0u16).to_socket_addrs() {
        Ok(addrs) => {
            let result: String = addrs
                .filter_map(|a| Some(a.ip().to_string()))
                .collect::<Vec<_>>()
                .join("\n");
            let bytes = result.as_bytes();
            let ptr = nimble_alloc(bytes.len() + 1);
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
                *ptr.add(bytes.len()) = 0;
            }
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_and_free() {
        let ptr = nimble_alloc(64);
        assert!(!ptr.is_null());
        unsafe { nimble_free(ptr, 64); }
    }

    #[test]
    fn string_ops() {
        unsafe {
            let a = nimble_string_new(b"hello" as *const u8, 5);
            let b = nimble_string_new(b"world" as *const u8, 5);
            let concat = nimble_string_concat(a.data, b.data);
            assert_eq!(to_cstr(concat), "helloworld");
            nimble_free(concat, 11);
            nimble_string_free(a);
            nimble_string_free(b);
        }
    }

    #[test]
    fn base64_roundtrip() {
        unsafe {
            let data = b"hello world";
            let encoded = nimble_base64_encode(data.as_ptr(), data.len() as i64, 0);
            assert!(!encoded.is_null());
            let enc_len = CStr::from_ptr(encoded as *const i8).to_bytes().len();
            let decoded = nimble_base64_decode(encoded, enc_len as i64);
            assert!(!decoded.is_null());
            let dec_len = CStr::from_ptr(decoded as *const i8).to_bytes().len();
            assert_eq!(CStr::from_ptr(decoded as *const i8).to_str().unwrap(), "hello world");
            nimble_free(encoded, enc_len + 1);
            nimble_free(decoded, dec_len + 1);
        }
    }

    #[test]
    fn hex_roundtrip() {
        unsafe {
            let data = b"\x01\x02\xFF";
            let encoded = nimble_hex_encode(data.as_ptr(), data.len() as i64, 1);
            assert!(!encoded.is_null());
            let enc_len = CStr::from_ptr(encoded as *const i8).to_bytes().len();
            assert_eq!(CStr::from_ptr(encoded as *const i8).to_str().unwrap(), "0102FF");
            let decoded = nimble_hex_decode(encoded, enc_len as i64);
            assert!(!decoded.is_null());
            let dec_len = CStr::from_ptr(decoded as *const i8).to_bytes().len();
            let dec_bytes = std::slice::from_raw_parts(decoded, dec_len);
            assert_eq!(dec_bytes, &[1, 2, 255]);
            nimble_free(encoded, enc_len + 1);
            nimble_free(decoded, dec_len + 1);
        }
    }

    #[test]
    fn hash_fnv1a_works() {
        unsafe {
            let data = b"hello";
            let h = nimble_hash_fnv1a(data.as_ptr(), data.len() as i64);
            assert_ne!(h, 0);
        }
    }

    #[test]
    fn math_functions() {
        assert!((nimble_sin(0.0) - 0.0).abs() < 1e-10);
        assert!((nimble_cos(0.0) - 1.0).abs() < 1e-10);
        assert!((nimble_sqrt(4.0) - 2.0).abs() < 1e-10);
        assert!((nimble_pow(2.0, 3.0) - 8.0).abs() < 1e-10);
        assert_eq!(nimble_abs_i64(-5), 5);
        assert_eq!(nimble_min_i64(3, 7), 3);
        assert_eq!(nimble_max_i64(3, 7), 7);
    }

    #[test]
    fn random_works() {
        let a = nimble_rand_i64();
        let b = nimble_rand_i64();
        assert_ne!(a, b);
    }

    #[test]
    fn int_to_string_works() {
        unsafe {
            let s = nimble_int_to_string(42);
            assert_eq!(to_cstr(s), "42");
            nimble_free(s, 3);
        }
    }

    #[test]
    fn string_find_works() {
        unsafe {
            let s = nimble_string_new(b"hello world" as *const u8, 11);
            let n = nimble_string_find(s.data, "world\0".as_ptr());
            assert_eq!(n, 6);
            nimble_string_free(s);
        }
    }

    #[test]
    fn file_exists_works() {
        unsafe {
            // Should return 0 for non-existent file
            let result = nimble_file_exists("/nonexistent_file_12345\0".as_ptr());
            assert_eq!(result, 0);
        }
    }

    #[test]
    fn utf8_validate() {
        unsafe {
            let valid = b"hello";
            assert_eq!(nimble_utf8_validate(valid.as_ptr(), valid.len() as i64), 1);
            let invalid = [0xFF, 0xFF];
            assert_eq!(nimble_utf8_validate(invalid.as_ptr(), invalid.len() as i64), 0);
        }
    }

    #[test]
    fn atomic_ops() {
        unsafe {
            let ptr = nimble_alloc(8);
            let p = ptr as *mut i64;
            nimble_atomic_store(p, 42);
            assert_eq!(nimble_atomic_load(p), 42);
            assert_eq!(nimble_atomic_add(p, 10), 42);
            assert_eq!(nimble_atomic_load(p), 52);
            nimble_free(ptr, 8);
        }
    }
}
