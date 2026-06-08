mod ffi {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        clippy::excessive_precision
    )]
    include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

pub use ffi::*;

pub type UpscalerContextDescription = ffi::FfxFsr3UpscalerContextDescription;
pub type UpscalerDispatchDescription = ffi::FfxFsr3UpscalerDispatchDescription;
pub type UpscalerGenerateReactiveDescription = ffi::FfxFsr3UpscalerGenerateReactiveDescription;
pub type UpscalerSharedResourceDescriptions = ffi::FfxFsr3UpscalerSharedResourceDescriptions;
pub type UpscalerConfigureKey = ffi::FfxFsr3UpscalerConfigureKey;
pub type UpscalerQualityMode = ffi::FfxFsr3UpscalerQualityMode;
pub type UpscalerMessage = ffi::FfxFsr3UpscalerMessage;
pub type EffectMemoryUsage = ffi::FfxEffectMemoryUsage;
pub type Dimensions2D = ffi::FfxDimensions2D;
pub type FloatCoords2D = ffi::FfxFloatCoords2D;

pub const FFX_OK: i32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FfxError(pub i32);

impl std::fmt::Display for FfxError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            FFX_OK => f.write_str("The operation completed successfully."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INVALID_POINTER as i32 => f.write_str("The operation failed due to an invalid pointer."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INVALID_ALIGNMENT as i32 => f.write_str("The operation failed due to an invalid alignment."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INVALID_SIZE as i32 => f.write_str("The operation failed due to an invalid size."),
            x if x == ffi::FfxErrorCodes::FFX_EOF as i32 => f.write_str("The end of the file was encountered."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INVALID_PATH as i32 => f.write_str("The operation failed because the specified path was invalid."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_EOF as i32 => f.write_str("The operation failed because end of file was reached."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_MALFORMED_DATA as i32 => f.write_str("The operation failed because of some malformed data."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_OUT_OF_MEMORY as i32 => f.write_str("The operation failed because it ran out memory."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INCOMPLETE_INTERFACE as i32 => f.write_str("The operation failed because the interface was not fully configured."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INVALID_ENUM as i32 => f.write_str("The operation failed because of an invalid enumeration value."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INVALID_ARGUMENT as i32 => f.write_str("The operation failed because an argument was invalid."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_OUT_OF_RANGE as i32 => f.write_str("The operation failed because a value was out of range."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_NULL_DEVICE as i32 => f.write_str("The operation failed because a device was null."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_BACKEND_API_ERROR as i32 => f.write_str("The operation failed because the backend API returned an error code."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INSUFFICIENT_MEMORY as i32 => f.write_str("The operation failed because there was not enough memory."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_INVALID_VERSION as i32 => f.write_str("The operation failed because the wrong backend was linked."),
            x if x == ffi::FfxErrorCodes::FFX_ERROR_ACCESS_DENIED as i32 => f.write_str("The operation failed because access to the resource was denied."),
            _ => write!(f, "Unknown FFX error code: {}", self.0),
        }
    }
}

impl std::error::Error for FfxError {}

#[inline]
pub fn ok_or_error(code: i32) -> Result<(), FfxError> {
    if code == FFX_OK {
        Ok(())
    } else {
        Err(FfxError(code))
    }
}

macro_rules! define_context {
    ($name:ident, $ctx:ty, $desc:ty, $dispatch_desc:ty, $create:path, $dispatch:path, $destroy:path) => {
        #[derive(Debug)]
        pub struct $name {
            inner: Box<$ctx>,
        }

        impl $name {
            pub fn create(description: &$desc) -> Result<Self, FfxError> {
                let mut ctx = unsafe { Box::<$ctx>::new_zeroed().assume_init() };
                ok_or_error(unsafe { $create(&mut *ctx, description as *const _ as *mut _) })?;
                Ok(Self { inner: ctx })
            }

            pub fn dispatch(&mut self, desc: &$dispatch_desc) -> Result<(), FfxError> {
                let code = unsafe { $dispatch(&mut *self.inner, desc) };
                ok_or_error(code)
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { $destroy(&mut *self.inner) };
            }
        }
    };
}

define_context!(
    UpscalerContext,
    ffi::FfxFsr3UpscalerContext,
    UpscalerContextDescription,
    UpscalerDispatchDescription,
    ffi::ffxFsr3UpscalerContextCreate,
    ffi::ffxFsr3UpscalerContextDispatch,
    ffi::ffxFsr3UpscalerContextDestroy
);

impl UpscalerContext {
    pub fn generate_reactive_mask(
        &mut self,
        params: &UpscalerGenerateReactiveDescription,
    ) -> Result<(), FfxError> {
        let code =
            unsafe { ffi::ffxFsr3UpscalerContextGenerateReactiveMask(&mut *self.inner, params) };
        ok_or_error(code)
    }

    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, FfxError> {
        let mut usage = ffi::FfxEffectMemoryUsage::default();
        ok_or_error(unsafe {
            ffi::ffxFsr3UpscalerContextGetGpuMemoryUsage(&mut *self.inner, &mut usage)
        })?;
        Ok(usage)
    }

    pub fn shared_resource_descriptions(
        &mut self,
    ) -> Result<UpscalerSharedResourceDescriptions, FfxError> {
        let mut desc = UpscalerSharedResourceDescriptions::default();
        ok_or_error(unsafe {
            ffi::ffxFsr3UpscalerGetSharedResourceDescriptions(&mut *self.inner, &mut desc)
        })?;
        Ok(desc)
    }

    pub unsafe fn set_constant(
        &mut self,
        key: UpscalerConfigureKey,
        value: *mut std::ffi::c_void,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxFsr3UpscalerSetConstant(&mut *self.inner, key, value) };
        ok_or_error(code)
    }

    pub fn set_float_constant(
        &mut self,
        key: UpscalerConfigureKey,
        value: f32,
    ) -> Result<(), FfxError> {
        unsafe { self.set_constant(key, &value as *const _ as *mut _) }
    }
}

define_context!(
    Fsr2Context,
    ffi::FfxFsr2Context,
    ffi::FfxFsr2ContextDescription,
    ffi::FfxFsr2DispatchDescription,
    ffi::ffxFsr2ContextCreate,
    ffi::ffxFsr2ContextDispatch,
    ffi::ffxFsr2ContextDestroy
);

impl Fsr2Context {
    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, FfxError> {
        let mut usage = ffi::FfxEffectMemoryUsage::default();
        ok_or_error(unsafe { ffi::ffxFsr2ContextGetGpuMemoryUsage(&mut *self.inner, &mut usage) })?;
        Ok(usage)
    }

    pub fn generate_reactive_mask(
        &mut self,
        desc: &ffi::FfxFsr2GenerateReactiveDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxFsr2ContextGenerateReactiveMask(&mut *self.inner, desc) };
        ok_or_error(code)
    }
}

define_context!(
    Fsr1Context,
    ffi::FfxFsr1Context,
    ffi::FfxFsr1ContextDescription,
    ffi::FfxFsr1DispatchDescription,
    ffi::ffxFsr1ContextCreate,
    ffi::ffxFsr1ContextDispatch,
    ffi::ffxFsr1ContextDestroy
);

impl Fsr1Context {
    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, FfxError> {
        let mut usage = ffi::FfxEffectMemoryUsage::default();
        ok_or_error(unsafe { ffi::ffxFsr1ContextGetGpuMemoryUsage(&mut *self.inner, &mut usage) })?;
        Ok(usage)
    }
}

define_context!(
    CasContext,
    ffi::FfxCasContext,
    ffi::FfxCasContextDescription,
    ffi::FfxCasDispatchDescription,
    ffi::ffxCasContextCreate,
    ffi::ffxCasContextDispatch,
    ffi::ffxCasContextDestroy
);

define_context!(
    CacaoContext,
    ffi::FfxCacaoContext,
    ffi::FfxCacaoContextDescription,
    ffi::FfxCacaoDispatchDescription,
    ffi::ffxCacaoContextCreate,
    ffi::ffxCacaoContextDispatch,
    ffi::ffxCacaoContextDestroy
);

impl CacaoContext {
    pub fn update_settings(
        &mut self,
        settings: &ffi::FfxCacaoSettings,
        use_downsampled_ssao: bool,
    ) -> Result<(), FfxError> {
        let code = unsafe {
            ffi::ffxCacaoUpdateSettings(&mut *self.inner, settings, use_downsampled_ssao)
        };
        ok_or_error(code)
    }
}

define_context!(
    DofContext,
    ffi::FfxDofContext,
    ffi::FfxDofContextDescription,
    ffi::FfxDofDispatchDescription,
    ffi::ffxDofContextCreate,
    ffi::ffxDofContextDispatch,
    ffi::ffxDofContextDestroy
);

define_context!(
    LensContext,
    ffi::FfxLensContext,
    ffi::FfxLensContextDescription,
    ffi::FfxLensDispatchDescription,
    ffi::ffxLensContextCreate,
    ffi::ffxLensContextDispatch,
    ffi::ffxLensContextDestroy
);

define_context!(
    LpmContext,
    ffi::FfxLpmContext,
    ffi::FfxLpmContextDescription,
    ffi::FfxLpmDispatchDescription,
    ffi::ffxLpmContextCreate,
    ffi::ffxLpmContextDispatch,
    ffi::ffxLpmContextDestroy
);

define_context!(
    SpdContext,
    ffi::FfxSpdContext,
    ffi::FfxSpdContextDescription,
    ffi::FfxSpdDispatchDescription,
    ffi::ffxSpdContextCreate,
    ffi::ffxSpdContextDispatch,
    ffi::ffxSpdContextDestroy
);

define_context!(
    ParallelSortContext,
    ffi::FfxParallelSortContext,
    ffi::FfxParallelSortContextDescription,
    ffi::FfxParallelSortDispatchDescription,
    ffi::ffxParallelSortContextCreate,
    ffi::ffxParallelSortContextDispatch,
    ffi::ffxParallelSortContextDestroy
);

define_context!(
    SssrContext,
    ffi::FfxSssrContext,
    ffi::FfxSssrContextDescription,
    ffi::FfxSssrDispatchDescription,
    ffi::ffxSssrContextCreate,
    ffi::ffxSssrContextDispatch,
    ffi::ffxSssrContextDestroy
);

define_context!(
    VrsContext,
    ffi::FfxVrsContext,
    ffi::FfxVrsContextDescription,
    ffi::FfxVrsDispatchDescription,
    ffi::ffxVrsContextCreate,
    ffi::ffxVrsContextDispatch,
    ffi::ffxVrsContextDestroy
);

define_context!(
    BlurContext,
    ffi::FfxBlurContext,
    ffi::FfxBlurContextDescription,
    ffi::FfxBlurDispatchDescription,
    ffi::ffxBlurContextCreate,
    ffi::ffxBlurContextDispatch,
    ffi::ffxBlurContextDestroy
);

define_context!(
    ClassifierContext,
    ffi::FfxClassifierContext,
    ffi::FfxClassifierContextDescription,
    ffi::FfxClassifierShadowDispatchDescription,
    ffi::ffxClassifierContextCreate,
    ffi::ffxClassifierContextShadowDispatch,
    ffi::ffxClassifierContextDestroy
);

impl ClassifierContext {
    pub fn dispatch_reflection(
        &mut self,
        desc: &ffi::FfxClassifierReflectionDispatchDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxClassifierContextReflectionDispatch(&mut *self.inner, desc) };
        ok_or_error(code)
    }
}

define_context!(
    DenoiserContext,
    ffi::FfxDenoiserContext,
    ffi::FfxDenoiserContextDescription,
    ffi::FfxDenoiserShadowsDispatchDescription,
    ffi::ffxDenoiserContextCreate,
    ffi::ffxDenoiserContextDispatchShadows,
    ffi::ffxDenoiserContextDestroy
);

impl DenoiserContext {
    pub fn dispatch_reflections(
        &mut self,
        desc: &ffi::FfxDenoiserReflectionsDispatchDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxDenoiserContextDispatchReflections(&mut *self.inner, desc) };
        ok_or_error(code)
    }
}

define_context!(
    OpticalFlowContext,
    ffi::FfxOpticalflowContext,
    ffi::FfxOpticalflowContextDescription,
    ffi::FfxOpticalflowDispatchDescription,
    ffi::ffxOpticalflowContextCreate,
    ffi::ffxOpticalflowContextDispatch,
    ffi::ffxOpticalflowContextDestroy
);

impl OpticalFlowContext {
    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, FfxError> {
        let mut usage = ffi::FfxEffectMemoryUsage::default();
        ok_or_error(unsafe {
            ffi::ffxOpticalflowContextGetGpuMemoryUsage(&mut *self.inner, &mut usage)
        })?;
        Ok(usage)
    }
}

/// A RAII wrapper around an `FfxDevice` obtained from a [`VkDeviceContext`].
///
/// The Vulkan backend stores a single global copy of the `VkDeviceContext`, so
/// creating multiple `Device`s will silently overwrite that global. No
/// destructor is needed — the application owns the Vulkan device lifetime.
pub struct Device {
    raw: ffi::FfxDevice,
    physical_device: ffi::VkPhysicalDevice,
}

impl Device {
    /// Creates a new device from a [`VkDeviceContext`].
    ///
    /// # Safety
    ///
    /// - `ctx` must contain valid Vulkan handles.
    /// - The backend's global device state will be overwritten.
    pub unsafe fn new(
        device: ffi::VkDevice,
        physical_device: ffi::VkPhysicalDevice,
    ) -> Result<Self, &'static str> {
        let mut ctx = ffi::VkDeviceContext {
            vkDevice: device,
            vkPhysicalDevice: physical_device,
            vkDeviceProcAddr: Some(vkGetDeviceProcAddr),
            instanceFunctions: ffi::VkInstanceFunctionTableFFX {
                getPhysicalDeviceFeatures2: Some(vkGetPhysicalDeviceFeatures2),
                enumerateDeviceExtensionProperties: Some(vkEnumerateDeviceExtensionProperties),
                getPhysicalDeviceMemoryProperties: Some(vkGetPhysicalDeviceMemoryProperties),
                getPhysicalDeviceProperties2: Some(vkGetPhysicalDeviceProperties2),
            },
        };
        let raw = unsafe { ffi::ffxGetDeviceVK((&mut ctx) as *mut _) };
        if raw.is_null() {
            Err("ffxGetDeviceVK returned null")
        } else {
            Ok(Self {
                raw,
                physical_device: ctx.vkPhysicalDevice,
            })
        }
    }

    pub fn as_raw(&self) -> ffi::FfxDevice {
        self.raw
    }
}

/// A RAII wrapper around a VK backend [`FfxInterface`] and its scratch buffer.
///
/// The scratch buffer must outlive any effect context created from this
/// interface, because the [`FfxInterface`] stores a raw pointer into it.
/// Rust's drop order guarantees this when the [`BackendInterface`] is held
/// alive for the duration of the effect context's use.
pub struct BackendInterface {
    inner: ffi::FfxInterface,
    _scratch: Vec<u8>,
}

impl BackendInterface {
    /// Creates a new backend interface.
    ///
    /// # Safety
    ///
    /// `device` must outlive this interface.
    pub unsafe fn new(device: &Device, max_contexts: usize) -> Result<Self, FfxError> {
        let scratch_size =
            unsafe { ffi::ffxGetScratchMemorySizeVK(device.physical_device, max_contexts) };
        let scratch = vec![0u8; scratch_size];
        let mut interface = ffi::FfxInterface::default();
        let code = unsafe {
            ffi::ffxGetInterfaceVK(
                &mut interface,
                device.raw,
                scratch.as_ptr() as *mut _,
                scratch_size,
                max_contexts,
            )
        };
        ok_or_error(code)?;
        Ok(Self {
            inner: interface,
            _scratch: scratch,
        })
    }

    pub fn as_ref(&self) -> &ffi::FfxInterface {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::FfxInterface {
        &mut self.inner
    }

    /// Creates an [`UpscalerContext`] from this backend interface.
    ///
    /// The [`BackendInterface`] must outlive the returned [`UpscalerContext`].
    pub fn create_upscaler(
        &self,
        flags: u32,
        render_size: Dimensions2D,
        upscale_size: Dimensions2D,
    ) -> Result<UpscalerContext, FfxError> {
        let desc = UpscalerContextDescription {
            flags,
            maxRenderSize: render_size,
            maxUpscaleSize: upscale_size,
            fpMessage: None,
            backendInterface: *self.as_ref(),
        };
        UpscalerContext::create(&desc)
    }
}

#[inline]
pub fn jitter_offset(index: i32, phase_count: i32) -> (f32, f32) {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    unsafe {
        ffi::ffxFsr3UpscalerGetJitterOffset(&mut x, &mut y, index, phase_count);
    }
    (x, y)
}

#[inline]
pub fn render_resolution_from_quality_mode(
    display_width: u32,
    display_height: u32,
    mode: UpscalerQualityMode,
) -> Result<(u32, u32), FfxError> {
    let mut w = 0u32;
    let mut h = 0u32;
    ok_or_error(unsafe {
        ffi::ffxFsr3UpscalerGetRenderResolutionFromQualityMode(
            &mut w,
            &mut h,
            display_width,
            display_height,
            mode,
        )
    })?;
    Ok((w, h))
}

// Helper function for VkDeviceContext
unsafe extern "C" {
    fn vkGetDeviceProcAddr(
        device: ffi::VkDevice,
        pName: *const std::ffi::c_char,
    ) -> ffi::PFN_vkVoidFunction;
    fn vkGetPhysicalDeviceFeatures2(
        physicalDevice: ffi::VkPhysicalDevice,
        pFeatures: *mut ffi::VkPhysicalDeviceFeatures2,
    );
    fn vkEnumerateDeviceExtensionProperties(
        physicalDevice: ffi::VkPhysicalDevice,
        pLayerName: *const std::ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut ffi::VkExtensionProperties,
    ) -> ffi::VkResult;
    fn vkGetPhysicalDeviceMemoryProperties(
        physicalDevice: ffi::VkPhysicalDevice,
        pMemoryProperties: *mut ffi::VkPhysicalDeviceMemoryProperties,
    );
    fn vkGetPhysicalDeviceProperties2(
        physicalDevice: ffi::VkPhysicalDevice,
        pProperties: *mut ffi::VkPhysicalDeviceProperties2,
    );
}

#[test]
fn context_creation() {
    use ash::vk::Handle;

    let entry = unsafe { ash::Entry::load() }.expect("failed to load vulkan");

    let app_info = ash::vk::ApplicationInfo::default().api_version(ash::vk::API_VERSION_1_3);
    let create_info = ash::vk::InstanceCreateInfo::default().application_info(&app_info);
    let instance =
        unsafe { entry.create_instance(&create_info, None) }.expect("failed to create instance");

    let physical_devices =
        unsafe { instance.enumerate_physical_devices() }.expect("enumerate_physical_devices");
    assert!(!physical_devices.is_empty(), "no physical devices");
    let physical_device = physical_devices[0];

    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let qfi = queue_families
        .iter()
        .position(|q| {
            q.queue_flags
                .contains(ash::vk::QueueFlags::GRAPHICS | ash::vk::QueueFlags::COMPUTE)
        })
        .expect("no graphics/compute queue family");

    let queue_info = ash::vk::DeviceQueueCreateInfo::default()
        .queue_family_index(qfi as u32)
        .queue_priorities(&[1.0]);
    let queue_create_infos = [queue_info];

    let ext_props = unsafe { instance.enumerate_device_extension_properties(physical_device) }
        .unwrap_or_default();
    let amd_coherent = ext_props.iter().any(|e| {
        e.extension_name_as_c_str()
            .is_ok_and(|name| name == ash::amd::device_coherent_memory::NAME)
    });

    let mut extension_names = vec![
        ash::khr::get_memory_requirements2::NAME.as_ptr(),
        ash::khr::dedicated_allocation::NAME.as_ptr(),
        ash::ext::shader_subgroup_ballot::NAME.as_ptr(),
    ];
    let mut amd_coherent_features = ash::vk::PhysicalDeviceCoherentMemoryFeaturesAMD::default();
    if amd_coherent {
        extension_names.push(ash::amd::device_coherent_memory::NAME.as_ptr());
        amd_coherent_features.device_coherent_memory = 1;
    }

    let mut features12 = ash::vk::PhysicalDeviceVulkan12Features::default()
        .shader_float16(true)
        .shader_sampled_image_array_non_uniform_indexing(true)
        .shader_subgroup_extended_types(true);
    let features = ash::vk::PhysicalDeviceFeatures::default()
        .shader_int16(true)
        .shader_image_gather_extended(true);
    let device_create_info = ash::vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(&extension_names)
        .push_next(&mut features12)
        .push_next(&mut amd_coherent_features)
        .enabled_features(&features);
    let device = unsafe { instance.create_device(physical_device, &device_create_info, None) }
        .expect("create_device");

    let ffx_device =
        unsafe { Device::new(device.handle().as_raw() as _, physical_device.as_raw() as _) }
            .expect("Device::new failed");
    assert!(!ffx_device.as_raw().is_null());

    let backend = unsafe { BackendInterface::new(&ffx_device, 17) }.unwrap();

    {
        let mut upscaler = backend
            .create_upscaler(
                FfxFsr3UpscalerInitializationFlagBits::FFX_FSR3UPSCALER_ENABLE_DEBUG_CHECKING,
                Dimensions2D {
                    width: 1280,
                    height: 720,
                },
                Dimensions2D {
                    width: 1280,
                    height: 720,
                },
            )
            .unwrap();

        dbg!(upscaler.gpu_memory_usage()).unwrap();
    }

    LpmContext::create(&ffi::FfxLpmContextDescription {
        flags: 0,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    SpdContext::create(&ffi::FfxSpdContextDescription {
        flags: 0,
        downsampleFilter: ffi::FfxSpdDownsampleFilter::FFX_SPD_DOWNSAMPLE_FILTER_MEAN,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    ParallelSortContext::create(&ffi::FfxParallelSortContextDescription {
        flags: 0,
        maxEntries: 1024,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    CasContext::create(&ffi::FfxCasContextDescription {
        flags: 0,
        colorSpaceConversion: ffi::FfxCasColorSpaceConversion::FFX_CAS_COLOR_SPACE_LINEAR,
        maxRenderSize: Dimensions2D {
            width: 1280,
            height: 720,
        },
        displaySize: Dimensions2D {
            width: 1280,
            height: 720,
        },
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    LensContext::create(&ffi::FfxLensContextDescription {
        flags: 0,
        outputFormat: ffi::FfxSurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
        floatPrecision: ffi::FfxLensFloatPrecision::FFX_LENS_FLOAT_PRECISION_32BIT,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    Fsr2Context::create(&ffi::FfxFsr2ContextDescription {
        flags: 0,
        maxRenderSize: Dimensions2D {
            width: 1280,
            height: 720,
        },
        displaySize: Dimensions2D {
            width: 1280,
            height: 720,
        },
        fpMessage: None,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    Fsr1Context::create(&ffi::FfxFsr1ContextDescription {
        flags: 0,
        outputFormat: ffi::FfxSurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
        maxRenderSize: Dimensions2D {
            width: 1280,
            height: 720,
        },
        displaySize: Dimensions2D {
            width: 1280,
            height: 720,
        },
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    CacaoContext::create(&ffi::FfxCacaoContextDescription {
        width: 1280,
        height: 720,
        useDownsampledSsao: false,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    DofContext::create(&ffi::FfxDofContextDescription {
        flags: 0,
        quality: 5,
        resolution: Dimensions2D {
            width: 1280,
            height: 720,
        },
        backendInterface: *backend.as_ref(),
        cocLimitFactor: 1.0,
    })
    .unwrap();

    SssrContext::create(&ffi::FfxSssrContextDescription {
        flags: 0,
        renderSize: Dimensions2D {
            width: 1280,
            height: 720,
        },
        normalsHistoryBufferFormat: ffi::FfxSurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    VrsContext::create(&ffi::FfxVrsContextDescription {
        flags: 0,
        shadingRateImageTileSize: 16,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    ClassifierContext::create(&ffi::FfxClassifierContextDescription {
        flags: ffi::FfxClassifierInitializationFlagBits::FFX_CLASSIFIER_SHADOW,
        resolution: Dimensions2D {
            width: 1280,
            height: 720,
        },
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    DenoiserContext::create(&ffi::FfxDenoiserContextDescription {
        flags: ffi::FfxDenoiserInitializationFlagBits::FFX_DENOISER_SHADOWS,
        windowSize: Dimensions2D {
            width: 1280,
            height: 720,
        },
        normalsHistoryBufferFormat: ffi::FfxSurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    OpticalFlowContext::create(&ffi::FfxOpticalflowContextDescription {
        backendInterface: *backend.as_ref(),
        flags: 0,
        resolution: Dimensions2D {
            width: 1280,
            height: 720,
        },
    })
    .unwrap();

    BlurContext::create(&ffi::FfxBlurContextDescription {
        kernelPermutations: ffi::FFX_BLUR_KERNEL_PERMUTATIONS_ALL,
        kernelSizes: ffi::FfxBlurKernelSize::FFX_BLUR_KERNEL_SIZE_3x3 as u32
            | ffi::FfxBlurKernelSize::FFX_BLUR_KERNEL_SIZE_5x5 as u32,
        floatPrecision: ffi::FfxBlurFloatPrecision::FFX_BLUR_FLOAT_PRECISION_32BIT,
        backendInterface: *backend.as_ref(),
    })
    .unwrap();

    unsafe { device.destroy_device(None) };
    unsafe { instance.destroy_instance(None) };
}
