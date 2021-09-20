#![no_main]
use libfuzzer_sys::fuzz_target;
use percent_encoding_rfc3986::{percent_decode};

// cargo +nightly fuzz run decode

fuzz_target!(|data: &[u8]| {

    if let Ok(decoded) = percent_decode(&data) {
        let _ = decoded.decode_utf8();
    }

});
