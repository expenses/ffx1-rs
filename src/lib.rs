#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdk_version() {
        assert_eq!(FFX_FSR3UPSCALER_VERSION_MAJOR, 3);
        assert_eq!(FFX_FSR3UPSCALER_VERSION_MINOR, 1);
        assert_eq!(FFX_FSR3UPSCALER_VERSION_PATCH, 4);
    }
}
