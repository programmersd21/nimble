//! NaN-boxed Value representation.
//!
//! A NaN-boxed value is a 64-bit float that encodes other types within its NaN payload.
//! 
//! Layout:
//! 1-bit sign, 11-bit exponent, 52-bit mantissa.
//!
//! - If exponent bits are all 1s (0x7FF), it's a special value (NaN/Infinity).
//! - Nimble uses a subset of NaN values to encode non-float types.
//! - Canonical NaN: 0x7FF8000000000000
//!
//! Type encoding in the 52-bit payload:
//! - 0xFFFE...: Pointer to heap-allocated object.
//! - 0xFFFF...: Immediate values (Bool, Null, Int).

 // Assuming this is kept as part of heap objects

#[repr(C)]
#[derive(Clone, Copy)]
pub union Value {
    pub f64_bits: u64,
    pub float: f64,
}

// NaN tagging constants (using bits)
const QNAN_MASK: u64 = 0x7ff8000000000000;
const TAG_NULL: u64 = 0x0000000000000001;
const TAG_BOOL: u64 = 0x0000000000000002;
const TAG_INT: u64  = 0x0000000000000003;

impl Value {
    pub fn from_f64(f: f64) -> Self {
        Self { float: f }
    }

    pub fn from_int(i: i64) -> Self {
        Self { f64_bits: QNAN_MASK | TAG_INT | ((i as u64) & 0xFFFFFFFF) }
    }

    pub fn from_bool(b: bool) -> Self {
        Self { f64_bits: QNAN_MASK | TAG_BOOL | (b as u64) }
    }

    pub fn from_null() -> Self {
        Self { f64_bits: QNAN_MASK | TAG_NULL }
    }

    pub fn is_float(&self) -> bool {
        unsafe { (self.f64_bits & QNAN_MASK) != QNAN_MASK }
    }

    pub fn as_f64(&self) -> f64 {
        unsafe { self.float }
    }

    pub fn as_int(&self) -> i64 {
        unsafe { (self.f64_bits & 0xFFFFFFFF) as i64 }
    }

    pub fn is_int(&self) -> bool {
        unsafe { (self.f64_bits & (QNAN_MASK | 0xFFFFFFFF00000000)) == (QNAN_MASK | TAG_INT) }
    }
}
