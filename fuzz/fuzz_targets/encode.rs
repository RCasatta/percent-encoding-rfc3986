#![no_main]
use libfuzzer_sys::fuzz_target;
use percent_encoding_rfc3986::{percent_encode, NON_ALPHANUMERIC};

// cargo +nightly fuzz run encode -- -only_ascii=1

fuzz_target!(|data: &[u8]| {
    let encode = percent_encode(data, NON_ALPHANUMERIC);
    let str = encode.to_string();

});
