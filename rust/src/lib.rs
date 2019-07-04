mod utils;
extern crate brotli;

extern crate libflate;
extern crate zip;
use brotli::*;
use std::io::Write;
use std::str;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = window)]
    fn result(s: &[u8]);
}

#[wasm_bindgen]
pub fn read_filename(filename: &str) {
    alert(&format!("Hello, {}!", filename));
}

#[wasm_bindgen]
pub fn read_bytes(filename: &str, array: &[u8]) {
    // We know these bytes are valid, so just use `unwrap()`.
    let content = str::from_utf8(array).unwrap();

    alert(&format!("{} file content is, {}!", filename, content));
}

#[wasm_bindgen]
pub fn to_zip(file_name: &str, file_content: &[u8]) -> std::vec::Vec<u8> {
    use std::vec::Vec;

    let buffer: &mut Vec<u8> = &mut Vec::new();
    compress_zip(buffer, file_name, file_content);

    buffer.to_vec()
}

fn compress_zip(output: &mut Vec<u8>, file_name: &str, input: &[u8]) -> zip::result::ZipResult<()> {
    let writer = std::io::Cursor::new(output);
    let mut zip = zip::ZipWriter::new(writer);

    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.start_file(file_name, options)?;
    zip.write(input)?;

    zip.finish()?;

    Ok(())
}

#[wasm_bindgen]
pub fn to_gzip(file_name: &str, file_content: &[u8]) -> std::vec::Vec<u8> {
    use std::vec::Vec;

    let buffer = compress_gzip(file_content);

    buffer.to_vec()
}

fn compress_gzip(input: &[u8]) -> std::vec::Vec<u8> {
    let mut reader = std::io::Cursor::new(input);


    let mut encoder = libflate::gzip::Encoder::new(Vec::new()).unwrap();
    std::io::copy(&mut reader, &mut encoder).unwrap();

    encoder.finish().into_result().unwrap()
}

#[wasm_bindgen]
pub fn to_brotli(_: &str, file_content: &mut [u8]) -> std::vec::Vec<u8> {
    use std::vec::Vec;

    let buffer: &mut Vec<u8> = &mut Vec::new();

    compress_brotli(buffer, file_content);

    buffer.to_vec()
}

fn compress_brotli(
    mut output: &mut Vec<u8>,
    input: &mut [u8],
) -> std::result::Result<(), std::io::Error> {
    let mut reader = std::io::Cursor::new(input);

    let params = enc::BrotliEncoderInitParams();
    match BrotliCompress(&mut reader, &mut output, &params) {
        Ok(_) => {}
        Err(_) => (),
    };

    Ok(())
}