#![no_main]
use libfuzzer_sys::fuzz_target;
use percent_encoding_rfc3986::{utf8_percent_encode, percent_encode, percent_decode_str, NON_ALPHANUMERIC};

// cargo +nightly fuzz run rt

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(original) => {
            let encoded = utf8_percent_encode(original, NON_ALPHANUMERIC).to_string();
            let decoded = percent_decode_str(&encoded).unwrap().decode_utf8().unwrap();
            assert_eq!(original, decoded);
        },
        Err(_) => {
            let encoded = percent_encode(data, NON_ALPHANUMERIC).to_string();
            let decoded = percent_decode_str(&encoded).unwrap();
            let decoded_bytes: std::borrow::Cow<[u8]> = decoded.into();
            assert_eq!(data, &decoded_bytes[..]);
        }
    }

});
