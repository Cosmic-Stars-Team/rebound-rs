use std::ffi::{CString, NulError};

use rebound_bind as rb;

/// Computes the REBOUND hash for `str_`.
///
/// This function is intentionally permissive: if `str_` contains an interior
/// NUL (`'\0'`), only the prefix before the first NUL is hashed. This matches
/// C-string semantics used by the underlying REBOUND C API.
///
/// If you need strict validation for interior NUL bytes, use [`try_hash`].
///
/// # Arguments
///
/// * `str_` - The string to hash.
///
/// # Returns
///
/// The REBOUND hash of `str_`.
///
/// # Examples
///
/// ```
/// assert_eq!(rebound::utils::hash("earth\0ignored"), rebound::utils::hash("earth"));
/// ```
pub fn hash(str_: &str) -> u32 {
    let head = str_.split('\0').next().unwrap_or_default();
    // SAFETY: `split('\0').next()` guarantees `head` has no interior NUL bytes.
    let cstr = unsafe { CString::from_vec_unchecked(head.as_bytes().to_vec()) };
    unsafe { rb::reb_hash(cstr.as_ptr()) }
}

/// Computes the REBOUND hash for `str_` with strict interior-NUL validation.
///
/// # Arguments
///
/// * `str_` - The string to hash.
///
/// # Returns
///
/// The REBOUND hash of `str_`.
///
/// # Errors
///
/// Returns an error if `str_` contains `'\0'`.
pub fn try_hash(str_: &str) -> Result<u32, NulError> {
    let cstr = CString::new(str_)?;
    unsafe { Ok(rb::reb_hash(cstr.as_ptr())) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_with_nul() {
        assert_eq!(hash("hello\0world"), hash("hello"));
        assert_eq!(hash("hello"), hash("hello"));
        assert!(try_hash("hello\0world").is_err());
        assert!(try_hash("hello, world").is_ok());
    }
}
