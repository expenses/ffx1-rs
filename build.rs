use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let ffx_src = out_dir.join("FidelityFX-SDK-Linux");
    // Copy the SDK to OUT_DIR before building because the SDK's CMakeLists.txt
    // hardcodes CMAKE_ARCHIVE_OUTPUT_DIRECTORY to the source tree
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

    // Search paths for all component libraries
    for dir in &[
        "backends/vk",
        "fsr3upscaler",
        "fsr2",
        "fsr1",
        "opticalflow",
        "spd",
        "cacao",
        "lpm",
        "blur",
        "vrs",
        "cas",
        "dof",
        "lens",
        "parallelsort",
        "denoiser",
        "sssr",
        "brixelizer",
        "brixelizergi",
        "classifier",
        "breadcrumbs",
    ] {
        println!(
            "cargo:rustc-link-search=native={}",
            build_dir.join("src").join("components").join(dir).display()
        );
    }
    println!(
        "cargo:rustc-link-search=native={}",
        build_dir.join("src").join("backends").join("vk").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        sdk.join("bin").join("ffx_sdk").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        ffx_src.join("bin").join("ffx_sdk").display()
    );

    // Link all component static libraries
    let libs = [
        "ffx_backend_vk_x64",
        "ffx_fsr3upscaler_x64",
        "ffx_fsr2_x64",
        "ffx_fsr1_x64",
        "ffx_opticalflow_x64",
        "ffx_spd_x64",
        "ffx_cacao_x64",
        "ffx_lpm_x64",
        "ffx_blur_x64",
        "ffx_vrs_x64",
        "ffx_cas_x64",
        "ffx_dof_x64",
        "ffx_lens_x64",
        "ffx_parallelsort_x64",
        "ffx_denoiser_x64",
        "ffx_sssr_x64",
        "ffx_brixelizer_x64",
        "ffx_brixelizergi_x64",
        "ffx_classifier_x64",
        "ffx_breadcrumbs_x64",
    ];
    for lib in &libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
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
        // Backend
        .clang_arg("-DFFX_BACKEND_VK")
        .clang_arg("-DVK_NO_PROTOTYPES")
        // Enable all components for correct header parsing
        .clang_arg("-DFFX_FSR3UPSCALER")
        .clang_arg("-DFFX_FSR2")
        .clang_arg("-DFFX_FSR1")
        .clang_arg("-DFFX_OPTICALFLOW")
        .clang_arg("-DFFX_SPD")
        .clang_arg("-DFFX_CACAO")
        .clang_arg("-DFFX_LPM")
        .clang_arg("-DFFX_BLUR")
        .clang_arg("-DFFX_VRS")
        .clang_arg("-DFFX_CAS")
        .clang_arg("-DFFX_DOF")
        .clang_arg("-DFFX_LENS")
        .clang_arg("-DFFX_PARALLELSORT")
        .clang_arg("-DFFX_DENOISER")
        .clang_arg("-DFFX_SSSR")
        .clang_arg("-DFFX_BRIXELIZER")
        .clang_arg("-DFFX_BRIXELIZERGI")
        .clang_arg("-DFFX_CLASSIFIER")
        .clang_arg("-DFFX_BREADCRUMBS")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .constified_enum_module(
            "FfxResourceUsage\
            |FfxResourceStates\
            |FfxBindStage\
            |FfxFsr3UpscalerInitializationFlagBits\
            |FfxFsr2InitializationFlagBits\
            |FfxOpticalflowInitializationFlagBits\
            |FfxFsr3UpscalerDispatchFlags\
            |FfxResourceFlags\
            |FfxCasInitializationFlagBits\
            |FfxCacaoInitializationFlagBits\
            |FfxCacaoDispatchFlagsBits\
            |FfxDofInitializationFlagBits\
            |FfxLensInitializationFlagBits\
            |FfxSpdInitializationFlagBits\
            |FfxVrsInitializationFlagBits\
            |FfxParallelSortInitializationFlagBits\
            |FfxDenoiserInitializationFlagBits\
            |FfxSssrInitializationFlagBits\
            |FfxClassifierInitializationFlagBits\
            |FfxBreadcrumbsInitializationFlagBits\
            |FfxFsr1InitializationFlagBits\
            |FfxBrixelizerGIFlags\
            |FfxBrixelizerCascadeFlag",
        )
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
