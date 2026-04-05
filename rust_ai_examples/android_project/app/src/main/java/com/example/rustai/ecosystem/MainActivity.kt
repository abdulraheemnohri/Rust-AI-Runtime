package com.example.rustai.ecosystem

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
                Log.e("RustAI", "Failed to load rust_runtime: \${e.message}")
            }
        }
    }

    external fun initRuntime(): Boolean
    external fun loadModel(modelPath: String, len: Int): Boolean
    external fun runInference(modelName: String, modelNameLen: Int, input: String, inputLen: Int): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        Log.d("RustAI", "Initializing Runtime...")
        initRuntime()

        val modelFile = File(filesDir, "sample_model.ggml")
        loadModel(modelFile.absolutePath, modelFile.absolutePath.length)

        val output = runInference("sample_model", 12, "Hello Rust AI Ecosystem", 23)
        Log.d("RustAI", "Output: \$output")
    }
}
