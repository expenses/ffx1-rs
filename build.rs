use std::path::PathBuf;

fn main() {
    let root = PathBuf::from("FidelityFX-SDK-Linux");
    let sdk = root.join("sdk");

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

    for dir in [
        build_dir.join("src").join("backends").join("vk"),
        build_dir.join("src").join("components").join("fsr2"),
        build_dir
            .join("src")
            .join("components")
            .join("fsr3upscaler"),
        build_dir.join("src").join("components").join("opticalflow"),
        build_dir
            .join("src")
            .join("components")
            .join("frameinterpolation"),
        sdk.join("bin").join("ffx_sdk"),
        root.join("bin").join("ffx_sdk"),
    ] {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }

    println!("cargo:rustc-link-lib=static=ffx_backend_vk_x64");
    println!("cargo:rustc-link-lib=static=ffx_fsr3upscaler_x64");
    println!("cargo:rustc-link-lib=static=ffx_fsr2_x64");
    println!("cargo:rustc-link-lib=static=ffx_opticalflow_x64");
    println!("cargo:rustc-link-lib=dylib=vulkan");

    let sdk_inc = sdk.join("include");
    let wrapper_h = std::env::current_dir().unwrap().join("wrapper.h");

    let bindgen_builder = bindgen::Builder::default()
        .header(wrapper_h.to_str().unwrap())
        .clang_arg("-xc++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{}", sdk_inc.display()))
        .allowlist_function("ffx.*")
        .allowlist_type("Ffx.*")
        .allowlist_type("ffx.*")
        .allowlist_var("FFX_.*")
        .clang_arg("-DFFX_BACKEND_VK")
        .clang_arg("-DVK_NO_PROTOTYPES")
        .clang_arg("-DFFX_FSR3UPSCALER")
        .clang_arg("-DFFX_FSR2")
        .derive_default(true)
        .generate_comments(false);

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_dir.join("ffi.rs")).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
}
