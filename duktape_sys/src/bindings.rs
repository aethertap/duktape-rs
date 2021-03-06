/* Automatically generated by rust-bindgen, and tweaked by hand. */

// We might be able to add derive(Copy) if it proves necessary.
#![allow(missing_copy_implementations)]

// Import more accurate versions of our typedefs.
use generated::*;

// Everything from here down is from bindgen, with uses of va_list
// commented out, and `extern "C"` replaced by `unsafe extern "C"`.
pub type duk_context = ::libc::c_void;
pub type duk_memory_functions = Struct_duk_memory_functions;
pub type duk_function_list_entry = Struct_duk_function_list_entry;
pub type duk_number_list_entry = Struct_duk_number_list_entry;
pub type duk_c_function =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut duk_context) -> duk_ret_t>;
pub type duk_alloc_function =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut ::libc::c_void, arg2: duk_size_t)
                              -> *mut ::libc::c_void>;
pub type duk_realloc_function =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut ::libc::c_void,
                               arg2: *mut ::libc::c_void, arg3: duk_size_t)
                              -> *mut ::libc::c_void>;
pub type duk_free_function =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut ::libc::c_void,
                               arg2: *mut ::libc::c_void)>;
pub type duk_fatal_function =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut duk_context, arg2: duk_errcode_t,
                               arg3: *const ::libc::c_char)>;
pub type duk_decode_char_function =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut ::libc::c_void,
                               arg2: duk_codepoint_t)>;
pub type duk_map_char_function =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut ::libc::c_void,
                               arg2: duk_codepoint_t) -> duk_codepoint_t>;
pub type duk_safe_call_function =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut duk_context) -> duk_ret_t>;
#[repr(C)]
pub struct Struct_duk_memory_functions {
    pub alloc_func: duk_alloc_function,
    pub realloc_func: duk_realloc_function,
    pub free_func: duk_free_function,
    pub udata: *mut ::libc::c_void,
}
#[repr(C)]
pub struct Struct_duk_function_list_entry {
    pub key: *const ::libc::c_char,
    pub value: duk_c_function,
    pub nargs: duk_idx_t,
}
#[repr(C)]
pub struct Struct_duk_number_list_entry {
    pub key: *const ::libc::c_char,
    pub value: duk_double_t,
}
extern "C" { }
extern "C" {
    pub fn duk_create_heap(alloc_func: duk_alloc_function,
                           realloc_func: duk_realloc_function,
                           free_func: duk_free_function,
                           alloc_udata: *mut ::libc::c_void,
                           fatal_handler: duk_fatal_function)
     -> *mut duk_context;
    pub fn duk_destroy_heap(ctx: *mut duk_context);
    pub fn duk_alloc_raw(ctx: *mut duk_context, size: duk_size_t)
     -> *mut ::libc::c_void;
    pub fn duk_free_raw(ctx: *mut duk_context, ptr: *mut ::libc::c_void);
    pub fn duk_realloc_raw(ctx: *mut duk_context, ptr: *mut ::libc::c_void,
                           size: duk_size_t) -> *mut ::libc::c_void;
    pub fn duk_alloc(ctx: *mut duk_context, size: duk_size_t)
     -> *mut ::libc::c_void;
    pub fn duk_free(ctx: *mut duk_context, ptr: *mut ::libc::c_void);
    pub fn duk_realloc(ctx: *mut duk_context, ptr: *mut ::libc::c_void,
                       size: duk_size_t) -> *mut ::libc::c_void;
    pub fn duk_get_memory_functions(ctx: *mut duk_context,
                                    out_funcs: *mut duk_memory_functions);
    pub fn duk_gc(ctx: *mut duk_context, flags: duk_uint_t);
    pub fn duk_throw(ctx: *mut duk_context);
    pub fn duk_error_raw(ctx: *mut duk_context, err_code: duk_errcode_t,
                         filename: *const ::libc::c_char, line: duk_int_t,
                         fmt: *const ::libc::c_char, ...);
    pub fn duk_fatal(ctx: *mut duk_context, err_code: duk_errcode_t,
                     err_msg: *const ::libc::c_char);
    pub fn duk_is_strict_call(ctx: *mut duk_context) -> duk_bool_t;
    pub fn duk_is_constructor_call(ctx: *mut duk_context) -> duk_bool_t;
    pub fn duk_normalize_index(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_idx_t;
    pub fn duk_require_normalize_index(ctx: *mut duk_context,
                                       index: duk_idx_t) -> duk_idx_t;
    pub fn duk_is_valid_index(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_require_valid_index(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_get_top(ctx: *mut duk_context) -> duk_idx_t;
    pub fn duk_set_top(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_get_top_index(ctx: *mut duk_context) -> duk_idx_t;
    pub fn duk_require_top_index(ctx: *mut duk_context) -> duk_idx_t;
    pub fn duk_check_stack(ctx: *mut duk_context, extra: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_require_stack(ctx: *mut duk_context, extra: duk_idx_t);
    pub fn duk_check_stack_top(ctx: *mut duk_context, top: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_require_stack_top(ctx: *mut duk_context, top: duk_idx_t);
    pub fn duk_swap(ctx: *mut duk_context, index1: duk_idx_t,
                    index2: duk_idx_t);
    pub fn duk_swap_top(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_dup(ctx: *mut duk_context, from_index: duk_idx_t);
    pub fn duk_dup_top(ctx: *mut duk_context);
    pub fn duk_insert(ctx: *mut duk_context, to_index: duk_idx_t);
    pub fn duk_replace(ctx: *mut duk_context, to_index: duk_idx_t);
    pub fn duk_copy(ctx: *mut duk_context, from_index: duk_idx_t,
                    to_index: duk_idx_t);
    pub fn duk_remove(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_xcopymove_raw(to_ctx: *mut duk_context,
                             from_ctx: *mut duk_context, count: duk_idx_t,
                             is_copy: duk_bool_t);
    pub fn duk_push_undefined(ctx: *mut duk_context);
    pub fn duk_push_null(ctx: *mut duk_context);
    pub fn duk_push_boolean(ctx: *mut duk_context, val: duk_bool_t);
    pub fn duk_push_true(ctx: *mut duk_context);
    pub fn duk_push_false(ctx: *mut duk_context);
    pub fn duk_push_number(ctx: *mut duk_context, val: duk_double_t);
    pub fn duk_push_nan(ctx: *mut duk_context);
    pub fn duk_push_int(ctx: *mut duk_context, val: duk_int_t);
    pub fn duk_push_uint(ctx: *mut duk_context, val: duk_uint_t);
    pub fn duk_push_string(ctx: *mut duk_context, str: *const ::libc::c_char)
     -> *const ::libc::c_char;
    pub fn duk_push_lstring(ctx: *mut duk_context, str: *const ::libc::c_char,
                            len: duk_size_t) -> *const ::libc::c_char;
    pub fn duk_push_pointer(ctx: *mut duk_context, p: *mut ::libc::c_void);
    pub fn duk_push_sprintf(ctx: *mut duk_context,
                            fmt: *const ::libc::c_char, ...)
     -> *const ::libc::c_char;
    //pub fn duk_push_vsprintf(ctx: *mut duk_context,
    //                         fmt: *const ::libc::c_char, ap: va_list)
    // -> *const ::libc::c_char;
    pub fn duk_push_string_file_raw(ctx: *mut duk_context,
                                    path: *const ::libc::c_char,
                                    flags: duk_uint_t)
     -> *const ::libc::c_char;
    pub fn duk_push_this(ctx: *mut duk_context);
    pub fn duk_push_current_function(ctx: *mut duk_context);
    pub fn duk_push_current_thread(ctx: *mut duk_context);
    pub fn duk_push_global_object(ctx: *mut duk_context);
    pub fn duk_push_heap_stash(ctx: *mut duk_context);
    pub fn duk_push_global_stash(ctx: *mut duk_context);
    pub fn duk_push_thread_stash(ctx: *mut duk_context,
                                 target_ctx: *mut duk_context);
    pub fn duk_push_object(ctx: *mut duk_context) -> duk_idx_t;
    pub fn duk_push_array(ctx: *mut duk_context) -> duk_idx_t;
    pub fn duk_push_c_function(ctx: *mut duk_context, func: duk_c_function,
                               nargs: duk_idx_t) -> duk_idx_t;
    pub fn duk_push_thread_raw(ctx: *mut duk_context, flags: duk_uint_t)
     -> duk_idx_t;
    pub fn duk_push_error_object_raw(ctx: *mut duk_context,
                                     err_code: duk_errcode_t,
                                     filename: *const ::libc::c_char,
                                     line: duk_int_t,
                                     fmt: *const ::libc::c_char, ...)
     -> duk_idx_t;
    pub fn duk_push_buffer(ctx: *mut duk_context, size: duk_size_t,
                           dynamic: duk_bool_t) -> *mut ::libc::c_void;
    pub fn duk_push_fixed_buffer(ctx: *mut duk_context, size: duk_size_t)
     -> *mut ::libc::c_void;
    pub fn duk_push_dynamic_buffer(ctx: *mut duk_context, size: duk_size_t)
     -> *mut ::libc::c_void;
    pub fn duk_pop(ctx: *mut duk_context);
    pub fn duk_pop_n(ctx: *mut duk_context, count: duk_idx_t);
    pub fn duk_pop_2(ctx: *mut duk_context);
    pub fn duk_pop_3(ctx: *mut duk_context);
    pub fn duk_get_type(ctx: *mut duk_context, index: duk_idx_t) -> duk_int_t;
    pub fn duk_check_type(ctx: *mut duk_context, index: duk_idx_t,
                          _type: duk_int_t) -> duk_bool_t;
    pub fn duk_get_type_mask(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_uint_t;
    pub fn duk_check_type_mask(ctx: *mut duk_context, index: duk_idx_t,
                               mask: duk_uint_t) -> duk_bool_t;
    pub fn duk_is_undefined(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_null(ctx: *mut duk_context, index: duk_idx_t) -> duk_bool_t;
    pub fn duk_is_null_or_undefined(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_boolean(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_number(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_nan(ctx: *mut duk_context, index: duk_idx_t) -> duk_bool_t;
    pub fn duk_is_string(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_object(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_buffer(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_pointer(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_array(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_function(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_c_function(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_ecmascript_function(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_bound_function(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_thread(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_callable(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_dynamic_buffer(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_fixed_buffer(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_is_primitive(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_get_boolean(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_get_number(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_double_t;
    pub fn duk_get_int(ctx: *mut duk_context, index: duk_idx_t) -> duk_int_t;
    pub fn duk_get_uint(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_uint_t;
    pub fn duk_get_string(ctx: *mut duk_context, index: duk_idx_t)
     -> *const ::libc::c_char;
    pub fn duk_get_lstring(ctx: *mut duk_context, index: duk_idx_t,
                           out_len: *mut duk_size_t) -> *const ::libc::c_char;
    pub fn duk_get_buffer(ctx: *mut duk_context, index: duk_idx_t,
                          out_size: *mut duk_size_t) -> *mut ::libc::c_void;
    pub fn duk_get_pointer(ctx: *mut duk_context, index: duk_idx_t)
     -> *mut ::libc::c_void;
    pub fn duk_get_c_function(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_c_function;
    pub fn duk_get_context(ctx: *mut duk_context, index: duk_idx_t)
     -> *mut duk_context;
    pub fn duk_get_length(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_size_t;
    pub fn duk_require_undefined(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_require_null(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_require_boolean(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_require_number(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_double_t;
    pub fn duk_require_int(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_int_t;
    pub fn duk_require_uint(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_uint_t;
    pub fn duk_require_string(ctx: *mut duk_context, index: duk_idx_t)
     -> *const ::libc::c_char;
    pub fn duk_require_lstring(ctx: *mut duk_context, index: duk_idx_t,
                               out_len: *mut duk_size_t)
     -> *const ::libc::c_char;
    pub fn duk_require_buffer(ctx: *mut duk_context, index: duk_idx_t,
                              out_size: *mut duk_size_t)
     -> *mut ::libc::c_void;
    pub fn duk_require_pointer(ctx: *mut duk_context, index: duk_idx_t)
     -> *mut ::libc::c_void;
    pub fn duk_require_c_function(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_c_function;
    pub fn duk_require_context(ctx: *mut duk_context, index: duk_idx_t)
     -> *mut duk_context;
    pub fn duk_to_undefined(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_to_null(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_to_boolean(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_to_number(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_double_t;
    pub fn duk_to_int(ctx: *mut duk_context, index: duk_idx_t) -> duk_int_t;
    pub fn duk_to_uint(ctx: *mut duk_context, index: duk_idx_t) -> duk_uint_t;
    pub fn duk_to_int32(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_int32_t;
    pub fn duk_to_uint32(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_uint32_t;
    pub fn duk_to_uint16(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_uint16_t;
    pub fn duk_to_string(ctx: *mut duk_context, index: duk_idx_t)
     -> *const ::libc::c_char;
    pub fn duk_to_lstring(ctx: *mut duk_context, index: duk_idx_t,
                          out_len: *mut duk_size_t) -> *const ::libc::c_char;
    pub fn duk_to_buffer(ctx: *mut duk_context, index: duk_idx_t,
                         out_size: *mut duk_size_t) -> *mut ::libc::c_void;
    pub fn duk_to_fixed_buffer(ctx: *mut duk_context, index: duk_idx_t,
                               out_size: *mut duk_size_t)
     -> *mut ::libc::c_void;
    pub fn duk_to_dynamic_buffer(ctx: *mut duk_context, index: duk_idx_t,
                                 out_size: *mut duk_size_t)
     -> *mut ::libc::c_void;
    pub fn duk_to_pointer(ctx: *mut duk_context, index: duk_idx_t)
     -> *mut ::libc::c_void;
    pub fn duk_to_object(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_to_defaultvalue(ctx: *mut duk_context, index: duk_idx_t,
                               hint: duk_int_t);
    pub fn duk_to_primitive(ctx: *mut duk_context, index: duk_idx_t,
                            hint: duk_int_t);
    pub fn duk_safe_to_lstring(ctx: *mut duk_context, index: duk_idx_t,
                               out_len: *mut duk_size_t)
     -> *const ::libc::c_char;
    pub fn duk_base64_encode(ctx: *mut duk_context, index: duk_idx_t)
     -> *const ::libc::c_char;
    pub fn duk_base64_decode(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_hex_encode(ctx: *mut duk_context, index: duk_idx_t)
     -> *const ::libc::c_char;
    pub fn duk_hex_decode(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_json_encode(ctx: *mut duk_context, index: duk_idx_t)
     -> *const ::libc::c_char;
    pub fn duk_json_decode(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_resize_buffer(ctx: *mut duk_context, index: duk_idx_t,
                             new_size: duk_size_t) -> *mut ::libc::c_void;
    pub fn duk_get_prop(ctx: *mut duk_context, obj_index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_get_prop_string(ctx: *mut duk_context, obj_index: duk_idx_t,
                               key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_get_prop_index(ctx: *mut duk_context, obj_index: duk_idx_t,
                              arr_index: duk_uarridx_t) -> duk_bool_t;
    pub fn duk_put_prop(ctx: *mut duk_context, obj_index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_put_prop_string(ctx: *mut duk_context, obj_index: duk_idx_t,
                               key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_put_prop_index(ctx: *mut duk_context, obj_index: duk_idx_t,
                              arr_index: duk_uarridx_t) -> duk_bool_t;
    pub fn duk_del_prop(ctx: *mut duk_context, obj_index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_del_prop_string(ctx: *mut duk_context, obj_index: duk_idx_t,
                               key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_del_prop_index(ctx: *mut duk_context, obj_index: duk_idx_t,
                              arr_index: duk_uarridx_t) -> duk_bool_t;
    pub fn duk_has_prop(ctx: *mut duk_context, obj_index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_has_prop_string(ctx: *mut duk_context, obj_index: duk_idx_t,
                               key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_has_prop_index(ctx: *mut duk_context, obj_index: duk_idx_t,
                              arr_index: duk_uarridx_t) -> duk_bool_t;
    pub fn duk_get_global_string(ctx: *mut duk_context,
                                 key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_put_global_string(ctx: *mut duk_context,
                                 key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_get_prototype(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_set_prototype(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_get_finalizer(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_set_finalizer(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_set_global_object(ctx: *mut duk_context);
    pub fn duk_get_magic(ctx: *mut duk_context, index: duk_idx_t)
     -> duk_int_t;
    pub fn duk_set_magic(ctx: *mut duk_context, index: duk_idx_t,
                         magic: duk_int_t);
    pub fn duk_get_current_magic(ctx: *mut duk_context) -> duk_int_t;
    pub fn duk_put_function_list(ctx: *mut duk_context, obj_index: duk_idx_t,
                                 funcs: *const duk_function_list_entry);
    pub fn duk_put_number_list(ctx: *mut duk_context, obj_index: duk_idx_t,
                               numbers: *const duk_number_list_entry);
    pub fn duk_get_var(ctx: *mut duk_context);
    pub fn duk_put_var(ctx: *mut duk_context);
    pub fn duk_del_var(ctx: *mut duk_context) -> duk_bool_t;
    pub fn duk_has_var(ctx: *mut duk_context) -> duk_bool_t;
    pub fn duk_compact(ctx: *mut duk_context, obj_index: duk_idx_t);
    pub fn duk_enum(ctx: *mut duk_context, obj_index: duk_idx_t,
                    enum_flags: duk_uint_t);
    pub fn duk_next(ctx: *mut duk_context, enum_index: duk_idx_t,
                    get_value: duk_bool_t) -> duk_bool_t;
    pub fn duk_concat(ctx: *mut duk_context, count: duk_idx_t);
    pub fn duk_join(ctx: *mut duk_context, count: duk_idx_t);
    pub fn duk_decode_string(ctx: *mut duk_context, index: duk_idx_t,
                             callback: duk_decode_char_function,
                             udata: *mut ::libc::c_void);
    pub fn duk_map_string(ctx: *mut duk_context, index: duk_idx_t,
                          callback: duk_map_char_function,
                          udata: *mut ::libc::c_void);
    pub fn duk_substring(ctx: *mut duk_context, index: duk_idx_t,
                         start_char_offset: duk_size_t,
                         end_char_offset: duk_size_t);
    pub fn duk_trim(ctx: *mut duk_context, index: duk_idx_t);
    pub fn duk_char_code_at(ctx: *mut duk_context, index: duk_idx_t,
                            char_offset: duk_size_t) -> duk_codepoint_t;
    pub fn duk_equals(ctx: *mut duk_context, index1: duk_idx_t,
                      index2: duk_idx_t) -> duk_bool_t;
    pub fn duk_strict_equals(ctx: *mut duk_context, index1: duk_idx_t,
                             index2: duk_idx_t) -> duk_bool_t;
    pub fn duk_call(ctx: *mut duk_context, nargs: duk_idx_t);
    pub fn duk_call_method(ctx: *mut duk_context, nargs: duk_idx_t);
    pub fn duk_call_prop(ctx: *mut duk_context, obj_index: duk_idx_t,
                         nargs: duk_idx_t);
    pub fn duk_pcall(ctx: *mut duk_context, nargs: duk_idx_t) -> duk_int_t;
    pub fn duk_pcall_method(ctx: *mut duk_context, nargs: duk_idx_t)
     -> duk_int_t;
    pub fn duk_pcall_prop(ctx: *mut duk_context, obj_index: duk_idx_t,
                          nargs: duk_idx_t) -> duk_int_t;
    pub fn duk_new(ctx: *mut duk_context, nargs: duk_idx_t);
    pub fn duk_safe_call(ctx: *mut duk_context, func: duk_safe_call_function,
                         nargs: duk_idx_t, nrets: duk_idx_t) -> duk_int_t;
    pub fn duk_eval_raw(ctx: *mut duk_context,
                        src_buffer: *const ::libc::c_char,
                        src_length: duk_size_t, flags: duk_uint_t)
     -> duk_int_t;
    pub fn duk_compile_raw(ctx: *mut duk_context,
                           src_buffer: *const ::libc::c_char,
                           src_length: duk_size_t, flags: duk_uint_t)
     -> duk_int_t;
    pub fn duk_log(ctx: *mut duk_context, level: duk_int_t,
                   fmt: *const ::libc::c_char, ...);
    pub fn duk_push_context_dump(ctx: *mut duk_context);
}
