use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let ffx_src = out_dir.join("FidelityFX-SDK-Linux");
    // Copy the SDK to OUT_DIR before building because the SDK's CMakeLists.txt
    // hardcodes CMAKE_ARCHIVE_OUTPUT_DIRECTORY to the source tree
    if !ffx_src.exists() {
        copy_dir_all(Path::new("FidelityFX-SDK-Linux"), &ffx_src)
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
    use bindgen::callbacks::{EnumVariantValue, ItemInfo, ParseCallbacks};

    #[derive(Debug)]
    struct Callbacks;

    impl ParseCallbacks for Callbacks {
        fn item_name(&self, info: ItemInfo) -> Option<String> {
            match info.name {
                "FfxResourceDescription__bindgen_ty_1" => return Some("SizeOrWidth".into()),
                "FfxResourceDescription__bindgen_ty_2" => return Some("HeightOrStride".into()),
                "FfxResourceDescription__bindgen_ty_3" => return Some("DepthOrAlignment".into()),
                _ => {}
            }
            if let Some(rest) = info.name.strip_prefix("Ffx") {
                return Some(rest.to_string());
            }
            if let Some(rest) = info.name.strip_prefix("ffx") {
                return Some(rest.to_string());
            }
            if let Some(rest) = info.name.strip_prefix("FFX_") {
                return Some(rest.to_string());
            }
            None
        }

        fn enum_variant_name(
            &self,
            _enum_name: Option<&str>,
            original_variant_name: &str,
            _variant_value: EnumVariantValue,
        ) -> Option<String> {
            const PREFIXES: &[&str] = &[
                "FFX_SURFACE_FORMAT_",
                "FFX_BRIXELIZER_GI_INTERNAL_",
                "FFX_SPD_DOWNSAMPLE_FILTER_",
                "FFX_CAS_COLOR_SPACE_",
                "FFX_LENS_FLOAT_",
                "FFX_BLUR_FLOAT_",
                "FFX_BLUR_KERNEL_",
                "FFX_BLUR_KERNEL_PERMUTATIONS_",
                "FFX_DENOISER_",
                "FFX_CLASSIFIER_",
                "FFX_FSR3UPSCALER_",
                "FFX_RESOURCE_TYPE_",
                "FFX_RESOURCE_FLAGS_",
                "FFX_RESOURCE_USAGE_",
                "FFX_RESOURCE_STATE_",
            ];
            for p in PREFIXES {
                if let Some(rest) = original_variant_name.strip_prefix(p) {
                    return Some(rest.to_string());
                }
            }
            if let Some(rest) = original_variant_name.strip_prefix("FFX_") {
                return Some(rest.to_string());
            }
            None
        }
    }

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
        .bitfield_enum("FfxBlurKernelSize")
        .bitfield_enum("FfxBlurKernelPermutation")
        .bitfield_enum("FfxResourceUsage")
        .bitfield_enum("FfxResourceStates")
        .bitfield_enum("FfxBindStage")
        .bitfield_enum("FfxFsr3UpscalerInitializationFlagBits")
        .bitfield_enum("FfxFsr2InitializationFlagBits")
        .bitfield_enum("FfxOpticalflowInitializationFlagBits")
        .bitfield_enum("FfxFsr3UpscalerDispatchFlags")
        .bitfield_enum("FfxResourceFlags")
        .bitfield_enum("FfxCasInitializationFlagBits")
        .bitfield_enum("FfxCacaoInitializationFlagBits")
        .bitfield_enum("FfxCacaoDispatchFlagsBits")
        .bitfield_enum("FfxDofInitializationFlagBits")
        .bitfield_enum("FfxLensInitializationFlagBits")
        .bitfield_enum("FfxSpdInitializationFlagBits")
        .bitfield_enum("FfxVrsInitializationFlagBits")
        .bitfield_enum("FfxParallelSortInitializationFlagBits")
        .bitfield_enum("FfxDenoiserInitializationFlagBits")
        .bitfield_enum("FfxSssrInitializationFlagBits")
        .bitfield_enum("FfxClassifierInitializationFlagBits")
        .bitfield_enum("FfxBreadcrumbsInitializationFlagBits")
        .bitfield_enum("FfxFsr1InitializationFlagBits")
        .bitfield_enum("FfxBrixelizerGIFlags")
        .bitfield_enum("FfxBrixelizerCascadeFlag")
        .bitfield_enum("FfxBlurKernelPermutation")
        .bitfield_enum("FfxBlurKernelSize")
        .derive_default(true)
        .generate_comments(true)
        .parse_callbacks(Box::new(Callbacks));

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
