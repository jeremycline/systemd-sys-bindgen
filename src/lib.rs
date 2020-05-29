#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod ffi;

#[cfg(test)]
mod tests {
    use crate::ffi::*;
    use std::io;
    use std::path::Path;

    #[test]
    fn test_sd_booted() {
        unsafe {
            let booted_with_sd = sd_booted();
            match Path::new("/run/systemd/system/").metadata() {
                Ok(_metadata) => {
                    assert!(booted_with_sd > 0);
                }
                Err(err) => {
                    if err.kind() == io::ErrorKind::NotFound {
                        assert!(booted_with_sd == 0)
                    } else {
                        assert!(booted_with_sd < 0)
                    }
                }
            }
        }
    }
}
