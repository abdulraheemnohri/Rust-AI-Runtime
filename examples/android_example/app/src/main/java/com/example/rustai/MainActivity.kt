package com.example.rustai

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import java.io.File

class MainActivity : AppCompatActivity() {

    companion object {
        init {
            try {
                System.loadLibrary("rust_runtime")
            } catch (e: UnsatisfiedLinkError) {
                Log.e("RustAI", "Failed to load native library: ${e.message}")
            }
        }
    }

    // Extern functions mapped to Rust
    external fun initRuntime(): Boolean
    external fun loadModel(modelPath: String, len: Int): Boolean
    external fun runInference(modelName: String, modelNameLen: Int, input: String, inputLen: Int): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // Note: For a minimal example, we don't have a layout file
        // setContentView(R.layout.activity_main)

        Log.d("RustAI", "Initializing Rust AI Runtime...")
        val initialized = initRuntime()
        Log.d("RustAI", "Runtime Initialized: $initialized")

        val modelFile = File(filesDir, "sample_model.ggml") // Example model path
        val modelLoaded = loadModel(modelFile.absolutePath, modelFile.absolutePath.length)
        Log.d("RustAI", "Model loaded: $modelLoaded")

        val input = "Hello Rust AI"
        val modelName = "sample_model"
        val output = runInference(modelName, modelName.length, input, input.length)
        Log.d("RustAI", "Inference Output: $output")
    }
}
