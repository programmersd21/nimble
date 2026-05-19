use crate::vm::{Value, VM};
use std::collections::HashMap;

pub mod core;
pub mod ffi;
pub mod io;
pub mod json;
pub mod math;
pub mod os;
pub mod regex;
pub mod time;
pub mod collections;
pub mod net;

pub fn install(globals: &mut HashMap<String, Value>) {
    let mut reg = |name: &str, f: fn(&mut VM, Vec<Value>) -> Value| {
        globals.insert(name.to_string(), Value::NativeFunction(f));
    };

    // Core
    reg("out", core::out);
    reg("in", core::input);
    reg("input", core::input);
    reg("len", core::len);
    reg("to_int", core::to_int);
    reg("index_of", core::index_of);
    reg("error", core::error);
    reg("__load_module", core::load_module);

    // IO
    reg("__builtin_file_exists", io::file_exists);
    reg("__builtin_io_read_file", io::read_file);
    reg("__builtin_io_write_file", io::write_file);
    reg("__builtin_io_append_file", io::append_file);
    reg("__builtin_io_delete_file", io::delete_file);
    reg("__builtin_io_read_lines", io::read_lines);
    reg("__builtin_io_write_lines", io::write_lines);
    reg("__builtin_io_read_bytes", io::read_bytes);
    reg("__builtin_io_write_bytes", io::write_bytes);
    reg("__builtin_io_copy_file", io::copy_file);
    reg("__builtin_io_stderr", io::stderr);

    // Math
    reg("__builtin_math_pow", math::pow);
    reg("__builtin_math_sqrt", math::sqrt);
    reg("__builtin_math_abs", math::abs);
    reg("__builtin_math_floor", math::floor);
    reg("__builtin_math_ceil", math::ceil);
    reg("__builtin_math_round", math::round);
    reg("__builtin_math_min", math::min);
    reg("__builtin_math_max", math::max);
    reg("__builtin_math_clamp", math::clamp);
    reg("__builtin_math_log", math::log);
    reg("__builtin_math_log2", math::log2);
    reg("__builtin_math_sin", math::sin);
    reg("__builtin_math_cos", math::cos);
    reg("__builtin_math_tan", math::tan);
    reg("__builtin_math_random", math::random);
    reg("__builtin_math_rand_int", math::rand_int);
    reg("__builtin_math_div", math::div);

    // FFI
    reg("__builtin_ffi_open", ffi::open);
    reg("__builtin_ffi_open_any", ffi::open_any);
    reg("__builtin_ffi_close", ffi::close);
    reg("__builtin_ffi_call", ffi::call);
    reg("__builtin_ffi_default_c", ffi::default_c);
    reg("__builtin_ffi_default_c_path", ffi::default_c_path);
    reg("__builtin_ffi_library_name", ffi::library_name);

    // OS / Path / Process
    reg("__builtin_os_args", os::args);
    reg("__builtin_os_exit", os::exit);
    reg("__builtin_os_env", os::env);
    reg("__builtin_path_join", os::path_join);
    reg("__builtin_process_run", os::process_run);

    // Regex
    reg("__builtin_regex_matches", regex::matches);
    reg("__builtin_regex_find", regex::find);
    reg("__builtin_regex_find_all", regex::find_all);
    reg("__builtin_regex_replace", regex::replace);
    reg("__builtin_regex_replace_all", regex::replace_all);
    reg("__builtin_regex_split", regex::split);

    // JSON
    reg("__builtin_json_parse", json::parse);
    reg("__builtin_json_stringify", json::stringify);
    reg("__builtin_json_pretty", json::pretty);

    // Time
    reg("__builtin_time_now", time::now);
    reg("__builtin_time_sleep", time::sleep);

    // Net
    reg("__builtin_net_http_get", net::http_get);

    // Collections - Map
    reg("__map_keys", collections::map_keys);
    reg("__builtin_map_has", collections::map_has);
    reg("__builtin_map_values", collections::map_values);
    reg("__builtin_map_merge", collections::map_merge);

    // Collections - List
    reg("__builtin_list_push", collections::list_push);
    reg("__builtin_list_pop", collections::list_pop);
    reg("__builtin_list_first", collections::list_first);
    reg("__builtin_list_last", collections::list_last);
    reg("__builtin_list_insert", collections::list_insert);
    reg("__builtin_list_remove", collections::list_remove);
    reg("__builtin_list_slice", collections::list_slice);
    reg("__builtin_list_sort", collections::list_sort);
    reg("__builtin_list_sort_inplace", collections::list_sort_inplace);
    reg("__builtin_list_reverse", collections::list_reverse);
    reg("__builtin_list_reverse_inplace", collections::list_reverse_inplace);
    reg("__builtin_list_contains", collections::list_contains);

    // Collections - String
    reg("__builtin_str_split", collections::str_split);
    reg("__builtin_str_join", collections::str_join);
    reg("__builtin_str_trim", collections::str_trim);
    reg("__builtin_str_trim_start", collections::str_trim_start);
    reg("__builtin_str_trim_end", collections::str_trim_end);
    reg("__builtin_str_upper", collections::str_upper);
    reg("__builtin_str_lower", collections::str_lower);
    reg("__builtin_str_starts_with", collections::str_starts_with);
    reg("__builtin_str_ends_with", collections::str_ends_with);
    reg("__builtin_str_replace", collections::str_replace);
    reg("__builtin_str_replace_all", collections::str_replace_all);
    reg("__builtin_str_count", collections::str_count);
    reg("__builtin_str_index_of", collections::str_index_of);
    reg("__builtin_str_slice", collections::str_slice);
    reg("__builtin_str_repeat", collections::str_repeat);
    reg("__builtin_str_pad_left", collections::str_pad_left);
    reg("__builtin_str_pad_right", collections::str_pad_right);
    reg("__builtin_str_to_int", collections::str_to_int);
    reg("__builtin_str_to_float", collections::str_to_float);
    reg("__builtin_str_from_int", collections::str_from_int);
    reg("__builtin_str_from_float", collections::str_from_float);
    reg("__builtin_str_chars", collections::str_chars);
    reg("__builtin_str_len", collections::str_len);
    reg("__builtin_str_is_numeric", collections::str_is_numeric);
    reg("__builtin_str_is_alpha", collections::str_is_alpha);
    reg("__builtin_str_format", collections::str_format);
    reg("__builtin_str_contains", collections::str_contains);
}
