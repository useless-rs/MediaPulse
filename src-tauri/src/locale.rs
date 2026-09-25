use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocaleError {
    #[error("could not set LC_NUMERIC to C: {0}")]
    SetNumeric(std::io::Error),
}

/// Ensure libmpv receives the numeric locale it requires.
#[allow(unsafe_code)]
pub fn set_numeric_c() -> Result<(), LocaleError> {
    // SAFETY: [Category 8 — FFI boundary] LC_NUMERIC is a valid libc category,
    // and c"C" is a static NUL-terminated string with the lifetime required by
    // setlocale. The returned pointer is only null-checked and never dereferenced.
    let result = unsafe { libc::setlocale(libc::LC_NUMERIC, c"C".as_ptr()) };
    if result.is_null() {
        return Err(LocaleError::SetNumeric(std::io::Error::last_os_error()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::set_numeric_c;

    #[allow(unsafe_code)]
    #[test]
    fn set_numeric_c_installs_c_locale() {
        set_numeric_c().expect("C locale should be available");

        // SAFETY: [Category 8 — FFI boundary] A null locale requests the current
        // locale pointer; it is only read after the null check below.
        let current = unsafe { libc::setlocale(libc::LC_NUMERIC, std::ptr::null()) };
        assert!(
            !current.is_null(),
            "setlocale must return the active locale"
        );

        // SAFETY: [Category 8 — FFI boundary] setlocale returned a non-null
        // pointer to a NUL-terminated locale string owned by libc.
        let current = unsafe { CStr::from_ptr(current) };
        assert_eq!(current.to_str().expect("locale must be UTF-8"), "C");
    }
}
