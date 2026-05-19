use crate::vm::{Value, VM};
use libffi::middle::{Arg, Cif, CodePtr, Type as LibffiType};
use libloading::Library;
use std::ffi::{c_char, c_void, CStr, CString};
use std::mem;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
enum FfiTypeDesc {
    Void,
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Isize,
    Usize,
    F32,
    F64,
    Pointer,
    CString,
}

struct CStringArg {
    _buf: CString,
    ptr: *const c_char,
}

enum FfiArgStorage {
    Bool(u8),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Pointer(*const c_void),
    CString(CStringArg),
}

impl FfiArgStorage {
    fn as_arg(&self) -> Arg {
        match self {
            Self::Bool(v) => Arg::new(v),
            Self::I8(v) => Arg::new(v),
            Self::U8(v) => Arg::new(v),
            Self::I16(v) => Arg::new(v),
            Self::U16(v) => Arg::new(v),
            Self::I32(v) => Arg::new(v),
            Self::U32(v) => Arg::new(v),
            Self::I64(v) => Arg::new(v),
            Self::U64(v) => Arg::new(v),
            Self::F32(v) => Arg::new(v),
            Self::F64(v) => Arg::new(v),
            Self::Pointer(v) => Arg::new(v),
            Self::CString(v) => Arg::new(&v.ptr),
        }
    }
}

impl FfiTypeDesc {
    fn from_name(name: &str) -> Result<Self, String> {
        match name.trim().to_ascii_lowercase().as_str() {
            "void" | "null" => Ok(Self::Void),
            "bool" => Ok(Self::Bool),
            "i8" => Ok(Self::I8),
            "u8" => Ok(Self::U8),
            "i16" => Ok(Self::I16),
            "u16" => Ok(Self::U16),
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "i64" | "int64" => Ok(Self::I64),
            "u64" | "uint64" => Ok(Self::U64),
            "isize" => Ok(Self::Isize),
            "usize" => Ok(Self::Usize),
            "f32" | "float" => Ok(Self::F32),
            "f64" | "double" => Ok(Self::F64),
            "ptr" | "pointer" => Ok(Self::Pointer),
            "str" | "cstring" => Ok(Self::CString),
            other => Err(format!("unsupported FFI type '{other}'")),
        }
    }

    fn ffi_type(&self) -> LibffiType {
        match self {
            Self::Void => LibffiType::void(),
            Self::Bool => LibffiType::u8(),
            Self::I8 => LibffiType::i8(),
            Self::U8 => LibffiType::u8(),
            Self::I16 => LibffiType::i16(),
            Self::U16 => LibffiType::u16(),
            Self::I32 => LibffiType::i32(),
            Self::U32 => LibffiType::u32(),
            Self::I64 | Self::Isize => {
                if mem::size_of::<isize>() == 8 {
                    LibffiType::i64()
                } else {
                    LibffiType::i32()
                }
            }
            Self::U64 | Self::Usize => {
                if mem::size_of::<usize>() == 8 {
                    LibffiType::u64()
                } else {
                    LibffiType::u32()
                }
            }
            Self::F32 => LibffiType::f32(),
            Self::F64 => LibffiType::f64(),
            Self::Pointer | Self::CString => LibffiType::pointer(),
        }
    }
}

fn ffi_string(value: &Value, ctx: &str) -> Result<String, String> {
    match value {
        Value::Str(s) => Ok(s.to_string()),
        _ => Err(format!("{ctx} must be string")),
    }
}

fn ffi_list(value: &Value, ctx: &str) -> Result<Vec<Value>, String> {
    match value {
        Value::List(items) => Ok(items.lock().unwrap().clone()),
        _ => Err(format!("{ctx} must be list")),
    }
}

fn ffi_library_path(value: &Value) -> Result<String, String> {
    match value {
        Value::FfiLibrary(path) | Value::Str(path) => Ok(path.to_string()),
        _ => Err("library must be an ffi handle or string path".into()),
    }
}

fn resolve_library_path(vm: &VM, path: &str) -> String {
    let library_path = PathBuf::from(path);
    if library_path.is_absolute() {
        return library_path.to_string_lossy().into_owned();
    }

    let has_explicit_relative = path.starts_with('.') || path.contains('/') || path.contains('\\');
    let module_relative = vm.current_module_dir_path().join(&library_path);

    if has_explicit_relative || module_relative.exists() {
        module_relative.to_string_lossy().into_owned()
    } else {
        path.to_string()
    }
}

fn parse_ffi_type_list(value: &Value, ctx: &str) -> Result<Vec<FfiTypeDesc>, String> {
    ffi_list(value, ctx)?
        .into_iter()
        .map(|item| match item {
            Value::Str(name) => FfiTypeDesc::from_name(&name),
            _ => Err(format!("{ctx} entries must be strings")),
        })
        .collect()
}

fn default_c_library_name() -> Result<String, String> {
    if cfg!(target_os = "windows") {
        Ok("msvcrt.dll".into())
    } else if cfg!(target_os = "macos") {
        Ok("/usr/lib/libSystem.B.dylib".into())
    } else if cfg!(target_os = "linux") {
        Ok("libc.so.6".into())
    } else {
        Err("ffi.default_c is not supported on this platform".into())
    }
}

fn platform_library_name(base: &str) -> Result<String, String> {
    let lower = base.to_ascii_lowercase();
    if lower.ends_with(".dll") || lower.ends_with(".so") || lower.ends_with(".dylib") {
        return Ok(base.to_string());
    }

    if cfg!(target_os = "windows") {
        Ok(format!("{base}.dll"))
    } else if cfg!(target_os = "macos") {
        let prefix = if base.starts_with("lib") { "" } else { "lib" };
        Ok(format!("{prefix}{base}.dylib"))
    } else if cfg!(target_os = "linux") {
        let prefix = if base.starts_with("lib") { "" } else { "lib" };
        Ok(format!("{prefix}{base}.so"))
    } else {
        Err("ffi.library_name is not supported on this platform".into())
    }
}

fn validate_ffi_library(path: &str) -> Result<(), String> {
    unsafe { Library::new(path) }
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn build_ffi_arg(expected: FfiTypeDesc, value: &Value) -> Result<FfiArgStorage, String> {
    match expected {
        FfiTypeDesc::Void => Err("void is not a valid argument type".into()),
        FfiTypeDesc::Bool => match value {
            Value::Bool(v) => Ok(FfiArgStorage::Bool(if *v { 1 } else { 0 })),
            Value::Int(v) => Ok(FfiArgStorage::Bool(if *v == 0 { 0 } else { 1 })),
            _ => Err("bool argument must be bool or int".into()),
        },
        FfiTypeDesc::I8 => match value {
            Value::Int(v) => i8::try_from(*v)
                .map(FfiArgStorage::I8)
                .map_err(|_| "i8 argument out of range".into()),
            _ => Err("i8 argument must be int".into()),
        },
        FfiTypeDesc::U8 => match value {
            Value::Int(v) => u8::try_from(*v)
                .map(FfiArgStorage::U8)
                .map_err(|_| "u8 argument out of range".into()),
            _ => Err("u8 argument must be int".into()),
        },
        FfiTypeDesc::I16 => match value {
            Value::Int(v) => i16::try_from(*v)
                .map(FfiArgStorage::I16)
                .map_err(|_| "i16 argument out of range".into()),
            _ => Err("i16 argument must be int".into()),
        },
        FfiTypeDesc::U16 => match value {
            Value::Int(v) => u16::try_from(*v)
                .map(FfiArgStorage::U16)
                .map_err(|_| "u16 argument out of range".into()),
            _ => Err("u16 argument must be int".into()),
        },
        FfiTypeDesc::I32 => match value {
            Value::Int(v) => i32::try_from(*v)
                .map(FfiArgStorage::I32)
                .map_err(|_| "i32 argument out of range".into()),
            _ => Err("i32 argument must be int".into()),
        },
        FfiTypeDesc::U32 => match value {
            Value::Int(v) => u32::try_from(*v)
                .map(FfiArgStorage::U32)
                .map_err(|_| "u32 argument out of range".into()),
            _ => Err("u32 argument must be int".into()),
        },
        FfiTypeDesc::I64 | FfiTypeDesc::Isize => match value {
            Value::Int(v) => Ok(FfiArgStorage::I64(*v)),
            _ => Err("i64/isize argument must be int".into()),
        },
        FfiTypeDesc::U64 | FfiTypeDesc::Usize => match value {
            Value::Int(v) => u64::try_from(*v)
                .map(FfiArgStorage::U64)
                .map_err(|_| "u64/usize argument out of range".into()),
            _ => Err("u64/usize argument must be int".into()),
        },
        FfiTypeDesc::F32 => match value {
            Value::Float(v) => Ok(FfiArgStorage::F32(*v as f32)),
            Value::Int(v) => Ok(FfiArgStorage::F32(*v as f32)),
            _ => Err("f32 argument must be number".into()),
        },
        FfiTypeDesc::F64 => match value {
            Value::Float(v) => Ok(FfiArgStorage::F64(*v)),
            Value::Int(v) => Ok(FfiArgStorage::F64(*v as f64)),
            _ => Err("f64 argument must be number".into()),
        },
        FfiTypeDesc::Pointer => match value {
            Value::Null => Ok(FfiArgStorage::Pointer(std::ptr::null())),
            Value::Int(v) if *v >= 0 => Ok(FfiArgStorage::Pointer(*v as usize as *const c_void)),
            Value::Int(_) => Err("pointer argument must be null or a non-negative int".into()),
            _ => Err("pointer argument must be null or int".into()),
        },
        FfiTypeDesc::CString => match value {
            Value::Str(s) => {
                let buf = CString::new(s.as_str())
                    .map_err(|_| "str argument contains an embedded NUL byte".to_string())?;
                let ptr = buf.as_ptr();
                Ok(FfiArgStorage::CString(CStringArg { _buf: buf, ptr }))
            }
            _ => Err("str argument must be string".into()),
        },
    }
}

unsafe fn ffi_invoke(
    library_path: &str,
    symbol_name: &str,
    arg_types: &[FfiTypeDesc],
    ret_type: FfiTypeDesc,
    args: &[Value],
) -> Result<Value, String> {
    if arg_types.len() != args.len() {
        return Err(format!(
            "FFI argument count mismatch: expected {}, got {}",
            arg_types.len(),
            args.len()
        ));
    }

    let library = Library::new(library_path).map_err(|err| err.to_string())?;
    let symbol_buf = CString::new(symbol_name)
        .map_err(|_| "symbol name contains an embedded NUL byte".to_string())?;
    let symbol = library
        .get::<*mut c_void>(symbol_buf.as_bytes_with_nul())
        .map_err(|err| err.to_string())?;

    let mut prepared = Vec::with_capacity(args.len());
    for (expected, value) in arg_types.iter().zip(args.iter()) {
        prepared.push(build_ffi_arg(*expected, value)?);
    }

    let ffi_args: Vec<Arg> = prepared.iter().map(FfiArgStorage::as_arg).collect();
    let cif = Cif::new(
        arg_types
            .iter()
            .map(FfiTypeDesc::ffi_type)
            .collect::<Vec<_>>(),
        ret_type.ffi_type(),
    );
    let code_ptr = CodePtr(*symbol);

    match ret_type {
        FfiTypeDesc::Void => {
            let _: () = cif.call(code_ptr, &ffi_args);
            Ok(Value::Null)
        }
        FfiTypeDesc::Bool => Ok(Value::Bool(cif.call::<u8>(code_ptr, &ffi_args) != 0)),
        FfiTypeDesc::I8 => Ok(Value::Int(cif.call::<i8>(code_ptr, &ffi_args) as i64)),
        FfiTypeDesc::U8 => Ok(Value::Int(cif.call::<u8>(code_ptr, &ffi_args) as i64)),
        FfiTypeDesc::I16 => Ok(Value::Int(cif.call::<i16>(code_ptr, &ffi_args) as i64)),
        FfiTypeDesc::U16 => Ok(Value::Int(cif.call::<u16>(code_ptr, &ffi_args) as i64)),
        FfiTypeDesc::I32 => Ok(Value::Int(cif.call::<i32>(code_ptr, &ffi_args) as i64)),
        FfiTypeDesc::U32 => Ok(Value::Int(cif.call::<u32>(code_ptr, &ffi_args) as i64)),
        FfiTypeDesc::I64 => Ok(Value::Int(cif.call::<i64>(code_ptr, &ffi_args))),
        FfiTypeDesc::U64 => Ok(Value::Int(cif.call::<u64>(code_ptr, &ffi_args) as i64)),
        FfiTypeDesc::Isize => {
            if mem::size_of::<isize>() == 8 {
                Ok(Value::Int(cif.call::<i64>(code_ptr, &ffi_args)))
            } else {
                Ok(Value::Int(cif.call::<i32>(code_ptr, &ffi_args) as i64))
            }
        }
        FfiTypeDesc::Usize => {
            if mem::size_of::<usize>() == 8 {
                Ok(Value::Int(cif.call::<u64>(code_ptr, &ffi_args) as i64))
            } else {
                Ok(Value::Int(cif.call::<u32>(code_ptr, &ffi_args) as i64))
            }
        }
        FfiTypeDesc::F32 => Ok(Value::Float(cif.call::<f32>(code_ptr, &ffi_args) as f64)),
        FfiTypeDesc::F64 => Ok(Value::Float(cif.call::<f64>(code_ptr, &ffi_args))),
        FfiTypeDesc::Pointer => Ok(Value::Int(
            cif.call::<*const c_void>(code_ptr, &ffi_args) as usize as i64
        )),
        FfiTypeDesc::CString => {
            let ptr = cif.call::<*const c_char>(code_ptr, &ffi_args);
            if ptr.is_null() {
                Ok(Value::Null)
            } else {
                Ok(Value::Str(Arc::new(
                    CStr::from_ptr(ptr).to_string_lossy().into_owned(),
                )))
            }
        }
    }
}

pub fn open(_vm: &mut VM, args: Vec<Value>) -> Value {
    let path = match args.get(0) {
        Some(value) => match ffi_library_path(value) {
            Ok(path) => path,
            Err(err) => return Value::Error(Arc::new(err)),
        },
        None => return Value::Error(Arc::new("ffi.open expects a library path".into())),
    };

    let resolved_path = resolve_library_path(_vm, &path);

    match validate_ffi_library(&resolved_path) {
        Ok(()) => Value::FfiLibrary(Arc::new(resolved_path)),
        Err(err) => Value::Error(Arc::new(err)),
    }
}

pub fn open_any(vm: &mut VM, args: Vec<Value>) -> Value {
    let candidates = match args.get(0) {
        Some(value) => match ffi_list(value, "paths") {
            Ok(paths) => paths,
            Err(err) => return Value::Error(Arc::new(err)),
        },
        None => return Value::Error(Arc::new("ffi.open_any expects a list of paths".into())),
    };

    let mut failures = Vec::new();
    for candidate in candidates {
        let raw = match ffi_string(&candidate, "paths entry") {
            Ok(path) => path,
            Err(err) => return Value::Error(Arc::new(err)),
        };
        let resolved = resolve_library_path(vm, &raw);
        match validate_ffi_library(&resolved) {
            Ok(()) => return Value::FfiLibrary(Arc::new(resolved)),
            Err(err) => failures.push(format!("{raw}: {err}")),
        }
    }

    Value::Error(Arc::new(format!(
        "failed to open any library candidate ({})",
        failures.join("; ")
    )))
}

pub fn close(_vm: &mut VM, args: Vec<Value>) -> Value {
    match args.get(0) {
        Some(Value::FfiLibrary(_)) | Some(Value::Str(_)) => Value::Null,
        Some(_) => Value::Error(Arc::new(
            "ffi.close expects an ffi handle or string path".into(),
        )),
        None => Value::Error(Arc::new(
            "ffi.close expects an ffi handle or string path".into(),
        )),
    }
}

pub fn default_c(_vm: &mut VM, _args: Vec<Value>) -> Value {
    let path = match default_c_library_name() {
        Ok(path) => path,
        Err(err) => return Value::Error(Arc::new(err)),
    };

    match validate_ffi_library(&path) {
        Ok(()) => Value::FfiLibrary(Arc::new(path)),
        Err(err) => Value::Error(Arc::new(err)),
    }
}

pub fn default_c_path(_vm: &mut VM, _args: Vec<Value>) -> Value {
    match default_c_library_name() {
        Ok(path) => Value::Str(Arc::new(path)),
        Err(err) => Value::Error(Arc::new(err)),
    }
}

pub fn library_name(_vm: &mut VM, args: Vec<Value>) -> Value {
    let base = match args.get(0) {
        Some(value) => match ffi_string(value, "base") {
            Ok(base) => base,
            Err(err) => return Value::Error(Arc::new(err)),
        },
        None => return Value::Error(Arc::new("ffi.library_name expects a base name".into())),
    };

    match platform_library_name(&base) {
        Ok(name) => Value::Str(Arc::new(name)),
        Err(err) => Value::Error(Arc::new(err)),
    }
}

pub fn call(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() != 5 {
        return Value::Error(Arc::new(
            "ffi.call expects library, symbol, arg_types, ret_type, args".into(),
        ));
    }

    let library_path = match ffi_library_path(&args[0]) {
        Ok(path) => path,
        Err(err) => return Value::Error(Arc::new(err)),
    };
    let library_path = match &args[0] {
        Value::FfiLibrary(_) => library_path,
        _ => resolve_library_path(_vm, &library_path),
    };
    let symbol_name = match ffi_string(&args[1], "symbol") {
        Ok(symbol) => symbol,
        Err(err) => return Value::Error(Arc::new(err)),
    };
    let arg_types = match parse_ffi_type_list(&args[2], "arg_types") {
        Ok(types) => types,
        Err(err) => return Value::Error(Arc::new(err)),
    };
    let ret_type =
        match ffi_string(&args[3], "ret_type").and_then(|name| FfiTypeDesc::from_name(&name)) {
            Ok(ret) => ret,
            Err(err) => return Value::Error(Arc::new(err)),
        };
    let call_args = match ffi_list(&args[4], "args") {
        Ok(args) => args,
        Err(err) => return Value::Error(Arc::new(err)),
    };

    match unsafe {
        ffi_invoke(
            &library_path,
            &symbol_name,
            &arg_types,
            ret_type,
            &call_args,
        )
    } {
        Ok(value) => value,
        Err(err) => Value::Error(Arc::new(err)),
    }
}
