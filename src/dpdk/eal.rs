use libc::{c_int, c_char};
use std::ffi::CString;
use std::ptr;

/// Represents errors that can occur during EAL operations.
#[derive(Debug)]
pub enum EalError {
    InitializationFailed,
    CleanupFailed,
}

/// Initializes the DPDK EAL with the given arguments.
///
/// # Arguments
/// * `args` - A vector of strings representing the EAL arguments.
///
/// # Returns
/// * `Ok(())` if initialization succeeds.
/// * `Err(EalError::InitializationFailed)` if initialization fails.
pub fn init(args: &[String]) -> Result<(), EalError> {
    // Convert Rust strings to C-style strings
    let c_args: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    // Convert CStrings to raw pointers
    let mut c_args_ptrs: Vec<*const c_char> = c_args.iter().map(|arg| arg.as_ptr()).collect();
    c_args_ptrs.push(ptr::null()); // Add null terminator

    // Call DPDK's `rte_eal_init` function
    let ret = unsafe { rte_eal_init(c_args_ptrs.len() as c_int, c_args_ptrs.as_ptr() as *mut *mut c_char) };

    if ret < 0 {
        Err(EalError::InitializationFailed)
    } else {
        Ok(())
    }
}

/// Cleans up the DPDK EAL.
///
/// # Returns
/// * `Ok(())` if cleanup succeeds.
/// * `Err(EalError::CleanupFailed)` if cleanup fails.
pub fn cleanup() -> Result<(), EalError> {
    let ret = unsafe { rte_eal_cleanup() };

    if ret < 0 {
        Err(EalError::CleanupFailed)
    } else {
        Ok(())
    }
}

/// FFI bindings for DPDK's EAL functions.
extern "C" {
    fn rte_eal_init(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn rte_eal_cleanup() -> c_int;
}