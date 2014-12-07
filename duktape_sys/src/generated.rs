//! Platform-specific values generated by defgen.c.

use libc::types::os::arch::c95::{c_long, c_double};

pub type duk_errcode_t = i32;
pub type duk_idx_t = i32;
pub type duk_int_t = i32;
pub type duk_ret_t = i32;
pub type duk_uint_t = u32;
pub type duk_bool_t = i32;
pub type duk_uarridx_t = u32;
pub type duk_codepoint_t = i32;
pub type duk_ucodepoint_t = u32;
pub type duk_size_t = u64;
pub type duk_ptrdiff_t = i64;
pub type duk_int32_t = i32;
pub type duk_uint32_t = u32;
pub type duk_uint16_t = u16;
pub type duk_double_t = c_double;
pub const DUK_VERSION: c_long = 10002;
pub const DUK_INVALID_INDEX: duk_idx_t = -2147483648;
pub const DUK_VARARGS: duk_int_t = -1;
pub const DUK_API_ENTRY_STACK: duk_idx_t = 64;
pub const DUK_TYPE_NONE: duk_int_t = 0;
pub const DUK_TYPE_UNDEFINED: duk_int_t = 1;
pub const DUK_TYPE_NULL: duk_int_t = 2;
pub const DUK_TYPE_BOOLEAN: duk_int_t = 3;
pub const DUK_TYPE_NUMBER: duk_int_t = 4;
pub const DUK_TYPE_STRING: duk_int_t = 5;
pub const DUK_TYPE_OBJECT: duk_int_t = 6;
pub const DUK_TYPE_BUFFER: duk_int_t = 7;
pub const DUK_TYPE_POINTER: duk_int_t = 8;
pub const DUK_TYPE_MASK_NONE: duk_uint_t = 1;
pub const DUK_TYPE_MASK_UNDEFINED: duk_uint_t = 2;
pub const DUK_TYPE_MASK_NULL: duk_uint_t = 4;
pub const DUK_TYPE_MASK_BOOLEAN: duk_uint_t = 8;
pub const DUK_TYPE_MASK_NUMBER: duk_uint_t = 16;
pub const DUK_TYPE_MASK_STRING: duk_uint_t = 32;
pub const DUK_TYPE_MASK_OBJECT: duk_uint_t = 64;
pub const DUK_TYPE_MASK_BUFFER: duk_uint_t = 128;
pub const DUK_TYPE_MASK_POINTER: duk_uint_t = 256;
pub const DUK_TYPE_MASK_THROW: duk_uint_t = 1024;
pub const DUK_HINT_NONE: duk_int_t = 0;
pub const DUK_HINT_STRING: duk_int_t = 1;
pub const DUK_HINT_NUMBER: duk_int_t = 2;
pub const DUK_ENUM_INCLUDE_NONENUMERABLE: duk_uint_t = 1;
pub const DUK_ENUM_INCLUDE_INTERNAL: duk_uint_t = 2;
pub const DUK_ENUM_OWN_PROPERTIES_ONLY: duk_uint_t = 4;
pub const DUK_ENUM_ARRAY_INDICES_ONLY: duk_uint_t = 8;
pub const DUK_ENUM_SORT_ARRAY_INDICES: duk_uint_t = 16;
pub const DUK_ENUM_NO_PROXY_BEHAVIOR: duk_uint_t = 32;
pub const DUK_COMPILE_EVAL: duk_uint_t = 1;
pub const DUK_COMPILE_FUNCTION: duk_uint_t = 2;
pub const DUK_COMPILE_STRICT: duk_uint_t = 4;
pub const DUK_COMPILE_SAFE: duk_uint_t = 8;
pub const DUK_COMPILE_NORESULT: duk_uint_t = 16;
pub const DUK_COMPILE_NOSOURCE: duk_uint_t = 32;
pub const DUK_COMPILE_STRLEN: duk_uint_t = 64;
pub const DUK_THREAD_NEW_GLOBAL_ENV: duk_uint_t = 1;
pub const DUK_STRING_PUSH_SAFE: duk_uint_t = 1;
pub const DUK_ERR_UNIMPLEMENTED_ERROR: duk_errcode_t = 50;
pub const DUK_ERR_UNSUPPORTED_ERROR: duk_errcode_t = 51;
pub const DUK_ERR_INTERNAL_ERROR: duk_errcode_t = 52;
pub const DUK_ERR_ALLOC_ERROR: duk_errcode_t = 53;
pub const DUK_ERR_ASSERTION_ERROR: duk_errcode_t = 54;
pub const DUK_ERR_API_ERROR: duk_errcode_t = 55;
pub const DUK_ERR_UNCAUGHT_ERROR: duk_errcode_t = 56;
pub const DUK_ERR_ERROR: duk_errcode_t = 100;
pub const DUK_ERR_EVAL_ERROR: duk_errcode_t = 101;
pub const DUK_ERR_RANGE_ERROR: duk_errcode_t = 102;
pub const DUK_ERR_REFERENCE_ERROR: duk_errcode_t = 103;
pub const DUK_ERR_SYNTAX_ERROR: duk_errcode_t = 104;
pub const DUK_ERR_TYPE_ERROR: duk_errcode_t = 105;
pub const DUK_ERR_URI_ERROR: duk_errcode_t = 106;
pub const DUK_RET_UNIMPLEMENTED_ERROR: duk_ret_t = -50;
pub const DUK_RET_UNSUPPORTED_ERROR: duk_ret_t = -51;
pub const DUK_RET_INTERNAL_ERROR: duk_ret_t = -52;
pub const DUK_RET_ALLOC_ERROR: duk_ret_t = -53;
pub const DUK_RET_ASSERTION_ERROR: duk_ret_t = -54;
pub const DUK_RET_API_ERROR: duk_ret_t = -55;
pub const DUK_RET_UNCAUGHT_ERROR: duk_ret_t = -56;
pub const DUK_RET_ERROR: duk_ret_t = -100;
pub const DUK_RET_EVAL_ERROR: duk_ret_t = -101;
pub const DUK_RET_RANGE_ERROR: duk_ret_t = -102;
pub const DUK_RET_REFERENCE_ERROR: duk_ret_t = -103;
pub const DUK_RET_SYNTAX_ERROR: duk_ret_t = -104;
pub const DUK_RET_TYPE_ERROR: duk_ret_t = -105;
pub const DUK_RET_URI_ERROR: duk_ret_t = -106;
pub const DUK_EXEC_SUCCESS: duk_int_t = 0;
pub const DUK_EXEC_ERROR: duk_int_t = 1;
pub const DUK_LOG_TRACE: duk_int_t = 0;
pub const DUK_LOG_DEBUG: duk_int_t = 1;
pub const DUK_LOG_INFO: duk_int_t = 2;
pub const DUK_LOG_WARN: duk_int_t = 3;
pub const DUK_LOG_ERROR: duk_int_t = 4;
pub const DUK_LOG_FATAL: duk_int_t = 5;