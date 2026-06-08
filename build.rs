use std::path::{Path, PathBuf};
use std::fs;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let ffx_src = out_dir.join("FidelityFX-SDK-Linux");
    if !ffx_src.exists() {
        copy_dir_all(&Path::new("FidelityFX-SDK-Linux"), &ffx_src)
            .unwrap_or_else(|e| panic!("failed to copy FidelityFX-SDK-Linux to OUT_DIR: {e}"));
    }
    let sdk = ffx_src.join("sdk");

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
        ffx_src.join("bin").join("ffx_sdk"),
    ] {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }

    println!("cargo:rustc-link-lib=static=ffx_backend_vk_x64");
    println!("cargo:rustc-link-lib=static=ffx_fsr3upscaler_x64");
    println!("cargo:rustc-link-lib=static=ffx_fsr2_x64");
    println!("cargo:rustc-link-lib=static=ffx_opticalflow_x64");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=dylib=vulkan");

    let sdk_inc = sdk.join("include");
    let wrapper_h = std::env::current_dir().unwrap().join("wrapper.h");

    use bindgen::EnumVariation;

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
        .default_enum_style(EnumVariation::Rust { non_exhaustive: false })
        .constified_enum_module("FfxResourceUsage|FfxResourceStates|FfxBindStage|FfxFsr3UpscalerInitializationFlagBits|FfxFsr2InitializationFlagBits|FfxOpticalflowInitializationFlagBits|FfxFsr3UpscalerDispatchFlags|FfxResourceFlags")
        .derive_default(true)
        .generate_comments(true);

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file(out_dir.join("ffi.rs")).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    let mut stack = vec![(src.to_path_buf(), dst.to_path_buf())];

    while let Some((src_dir, dst_dir)) = stack.pop() {
        fs::create_dir_all(&dst_dir)?;
        for entry in fs::read_dir(&src_dir)? {
            let entry = entry?;
            let name = entry.file_name();
            if name == ".git" {
                continue;
            }
            let src_path = entry.path();
            let dst_path = dst_dir.join(&name);
            if entry.file_type()?.is_dir() {
                stack.push((src_path, dst_path));
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
    }

    Ok(())
}
