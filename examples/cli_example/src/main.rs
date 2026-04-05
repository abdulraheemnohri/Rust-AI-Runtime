use rust_runtime::{init_runtime, load_model, run_inference};
use std::ffi::CString;

fn main() {
    env_logger::init();

    println!("--- Rust AI Runtime CLI Example ---");

    if init_runtime() {
        println!("Runtime initialized successfully.");
    } else {
        println!("Failed to initialize runtime.");
        return;
    }

    let model_path = "models/sample_model.ggml";
    if load_model(model_path.as_ptr(), model_path.len()) {
        println!("Model loaded: {}", model_path);
    } else {
        println!("Failed to load model.");
    }

    let model_name = "sample_model";
    let input = "Hello from CLI";

    let output_ptr = run_inference(
        model_name.as_ptr(), model_name.len(),
        input.as_ptr(), input.len()
    );

    if !output_ptr.is_null() {
        let output_cstr = unsafe { CString::from_raw(output_ptr) };
        println!("Output: {:?}", output_cstr.to_str().unwrap());
    } else {
        println!("Inference failed.");
    }
}
