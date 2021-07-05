// https://github.com/endeav0r/corpora/blob/c6da29474bdba79e7cf34b4475141cf0ff80d0a3/src/stack_buffer/vuln/one.c
const BINARY_0_BASE64: &str = "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";

#[cfg(test)]
pub(crate) fn get_test_binary_0() -> crate::rc::Ref<crate::binaryview::BinaryView> {
    use std::io::Write;
    let bytes = miniz_oxide::inflate::decompress_to_vec_zlib(
        &base64::decode(BINARY_0_BASE64).expect("Failed to base64 decode binary_0"),
    )
    .expect("Failed to decompress binary_0");

    let mut file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    file.as_file_mut()
        .write_all(&bytes)
        .expect("Failed to write to temporary file");

    file.flush().expect("Failed to flush temporary file");

    crate::open_view(file.path()).expect("Failed to create view")

    // crate::open_view("/Users/dev/code/binaryninja-api/rust/binaryninja/one").expect("Failed to create binary view")
}

#[test]
fn open_test_binary_0() {
    crate::headless::init();

    get_test_binary_0();
}
