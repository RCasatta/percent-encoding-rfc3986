#![no_main]
use libfuzzer_sys::fuzz_target;
use percent_encoding_rfc3986::{utf8_percent_encode, percent_decode_str, NON_ALPHANUMERIC};

// cargo +nightly fuzz run rt -- -only_ascii=1

fuzz_target!(|data: &[u8]| {
    let original = std::str::from_utf8(data).unwrap();
    let encoded = utf8_percent_encode(original, NON_ALPHANUMERIC).to_string();
    let decoded = percent_decode_str(&encoded).unwrap().decode_utf8().unwrap();
    assert_eq!(original, decoded);
});
