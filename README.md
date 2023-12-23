# Toxoid Engine
*⚠️ Warning ⚠️ - This repository is under construction.*


# Web Target
Setup emscripten: https://emscripten.org/docs/getting_started/downloads.html

# Examples
How to run examples:
```
rustup target add wasm32-unknown-emscripten
make example-snake
```

# Credits
This technology would not be possible without:
- [Flecs](https://github.com/SanderMertens/flecs): A fast, lightweight, and portable zero dependency entity component system written in C.
- [Sokol](https://github.com/floooh/sokol): Headers-only cross-platform C renderer with WebGPU support.
- [Emscripten](https://github.com/emscripten-core/emsdk): A complete compiler toolchain to WebAssembly, using LLVM, with a special focus on speed, size, and the Web platform.
- [Rust](https://www.rust-lang.org/): A memory-safe systems programming language. 
