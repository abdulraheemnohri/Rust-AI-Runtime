use rust_runtime::{init_runtime, run_inference, call_plugin};
use std::ffi::CString;

fn main() {
    println!("--- Rust AI Runtime Ecosystem CLI ---");

    if init_runtime() {
        println!("Runtime initialized.");
    }

    // Core inference example
    let model_name = "sample_model";
    let input = "Hello from CLI";
    let output_ptr = run_inference(
        model_name.as_ptr(), model_name.len(),
        input.as_ptr(), input.len()
    );
    if !output_ptr.is_null() {
        let output = unsafe { CString::from_raw(output_ptr) };
        println!("Core AI Output: {:?}", output.to_str().unwrap());
    }

    // Plugin loading example
    #[cfg(target_os = "linux")]
    let plugin_path = "./libvoice_assistant.so";
    #[cfg(target_os = "macos")]
    let plugin_path = "./libvoice_assistant.dylib";
    #[cfg(target_os = "windows")]
    let plugin_path = "./voice_assistant.dll";

    let symbol_name = "run_voice_assistant";
    let plugin_input = "Voice command: what is the weather?";

    println!("Attempting to call plugin: {} at {}", symbol_name, plugin_path);

    let plugin_output_ptr = call_plugin(
        plugin_path.as_ptr(), plugin_path.len(),
        symbol_name.as_ptr(), symbol_name.len(),
        plugin_input.as_ptr(), plugin_input.len()
    );

    if !plugin_output_ptr.is_null() {
        let plugin_output = unsafe { CString::from_raw(plugin_output_ptr) };
        println!("Plugin Output: {:?}", plugin_output.to_str().unwrap());
    } else {
        println!("Plugin call failed.");
    }
}
