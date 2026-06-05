use std::path::PathBuf;

fn main() {
    let root = PathBuf::from("FidelityFX-SDK-Linux");
    let sdk = root.join("sdk");

    // --- Build the SDK ---
    let mut cmake_cfg = cmake::Config::new(&sdk);
    cmake_cfg
        .define("FFX_API_BACKEND", "VK_X64")
        .define("FFX_ALL", "ON")
        .define("FFX_AUTO_COMPILE_SHADERS", "OFF")
        .define("FFX_BUILD_AS_DLL", "OFF")
        .profile("Release")
        .build_target("all");

    let dst = cmake_cfg.build();
    let build_dir = dst.join("build");

    // --- Search for built static libraries ---
    // Search both the build tree and the configured output directories
    let search_paths = [
        build_dir.join("src").join("backends").join("vk"),
        build_dir.join("src").join("components").join("fsr2"),
        build_dir.join("src").join("components").join("fsr3upscaler"),
        build_dir.join("src").join("components").join("opticalflow"),
        build_dir.join("src").join("components").join("frameinterpolation"),
        sdk.join("bin").join("ffx_sdk"),
        root.join("bin").join("ffx_sdk"),
    ];

    for dir in &search_paths {
        if dir.exists() {
            println!("cargo:rustc-link-search=native={}", dir.display());
        }
    }

    // --- Link the required libraries ---
    println!("cargo:rustc-link-lib=static=ffx_backend_vk_x64");
    println!("cargo:rustc-link-lib=static=ffx_fsr3upscaler_x64");
    println!("cargo:rustc-link-lib=static=ffx_fsr2_x64");
    println!("cargo:rustc-link-lib=static=ffx_opticalflow_x64");
    println!("cargo:rustc-link-lib=dylib=vulkan");

    // --- Generate Rust FFI bindings ---
    let sdk_inc = sdk.join("include");
    let wrapper_h = std::env::current_dir().unwrap().join("wrapper.h");

    let mut bindgen_builder = bindgen::Builder::default()
        .header(wrapper_h.to_str().unwrap())
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg(format!("-I{}", sdk_inc.display()))
        .allowlist_function("ffx.*")
        .allowlist_type("Ffx.*")
        .allowlist_type("ffx.*")
        .allowlist_var("FFX_.*")
        .derive_default(true)
        .generate_comments(false);

    let cxx_include = std::process::Command::new("g++")
        .args(["-E", "-x", "c++", "-", "-v"])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .output()
        .ok()
        .and_then(|out| {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let mut paths = Vec::new();
            for line in stderr.lines() {
                let line = line.trim();
                if line.starts_with('/') && line.contains("c++") {
                    paths.push(line.to_string());
                }
            }
            // We need the main C++ include dir and the platform-specific one
            paths.retain(|p| !p.contains("backward"));
            (!paths.is_empty()).then(|| paths)
        })
        .unwrap_or_default();

    for path in &cxx_include {
        bindgen_builder = bindgen_builder.clang_arg(format!("-isystem{}", path));
    }

    for def in [
        "FFX_BACKEND_VK",
        "VK_NO_PROTOTYPES",
        "FFX_FSR2",
        "FFX_FSR3UPSCALER",
    ] {
        bindgen_builder = bindgen_builder.clang_arg(format!("-D{}", def));
    }

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("ffi.rs"))
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
}
