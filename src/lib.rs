pub mod ffi {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        clippy::excessive_precision
    )]
    include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

use ffi::*;

pub const FFX_OK: i32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FfxError(pub i32);

impl std::fmt::Display for FfxError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            FFX_OK => f.write_str("The operation completed successfully."),
            x if x == ErrorCodes::FFX_ERROR_INVALID_POINTER as i32 => f.write_str("The operation failed due to an invalid pointer."),
            x if x == ErrorCodes::FFX_ERROR_INVALID_ALIGNMENT as i32 => f.write_str("The operation failed due to an invalid alignment."),
            x if x == ErrorCodes::FFX_ERROR_INVALID_SIZE as i32 => f.write_str("The operation failed due to an invalid size."),
            x if x == ErrorCodes::FFX_EOF as i32 => f.write_str("The end of the file was encountered."),
            x if x == ErrorCodes::FFX_ERROR_INVALID_PATH as i32 => f.write_str("The operation failed because the specified path was invalid."),
            x if x == ErrorCodes::FFX_ERROR_EOF as i32 => f.write_str("The operation failed because end of file was reached."),
            x if x == ErrorCodes::FFX_ERROR_MALFORMED_DATA as i32 => f.write_str("The operation failed because of some malformed data."),
            x if x == ErrorCodes::FFX_ERROR_OUT_OF_MEMORY as i32 => f.write_str("The operation failed because it ran out memory."),
            x if x == ErrorCodes::FFX_ERROR_INCOMPLETE_INTERFACE as i32 => f.write_str("The operation failed because the interface was not fully configured."),
            x if x == ErrorCodes::FFX_ERROR_INVALID_ENUM as i32 => f.write_str("The operation failed because of an invalid enumeration value."),
            x if x == ErrorCodes::FFX_ERROR_INVALID_ARGUMENT as i32 => f.write_str("The operation failed because an argument was invalid."),
            x if x == ErrorCodes::FFX_ERROR_OUT_OF_RANGE as i32 => f.write_str("The operation failed because a value was out of range."),
            x if x == ErrorCodes::FFX_ERROR_NULL_DEVICE as i32 => f.write_str("The operation failed because a device was null."),
            x if x == ErrorCodes::FFX_ERROR_BACKEND_API_ERROR as i32 => f.write_str("The operation failed because the backend API returned an error code."),
            x if x == ErrorCodes::FFX_ERROR_INSUFFICIENT_MEMORY as i32 => f.write_str("The operation failed because there was not enough memory."),
            x if x == ErrorCodes::FFX_ERROR_INVALID_VERSION as i32 => f.write_str("The operation failed because the wrong backend was linked."),
            x if x == ErrorCodes::FFX_ERROR_ACCESS_DENIED as i32 => f.write_str("The operation failed because access to the resource was denied."),
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
    ($name:ident, $ctx:ty, $desc:ty, $create:path, $destroy:path) => {
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
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { $destroy(&mut *self.inner) };
            }
        }
    };
    ($name:ident, $ctx:ty, $desc:ty, $dispatch_desc:ty, $create:path, $dispatch:path, $destroy:path) => {
        define_context!($name, $ctx, $desc, $create, $destroy);

        impl $name {
            pub fn dispatch(&mut self, desc: &$dispatch_desc) -> Result<(), FfxError> {
                let code = unsafe { $dispatch(&mut *self.inner, desc) };
                ok_or_error(code)
            }
        }
    };
}

define_context!(
    Fsr3UpscalerContext,
    ffi::Fsr3UpscalerContext,
    Fsr3UpscalerContextDescription,
    Fsr3UpscalerDispatchDescription,
    Fsr3UpscalerContextCreate,
    Fsr3UpscalerContextDispatch,
    Fsr3UpscalerContextDestroy
);

impl Fsr3UpscalerContext {
    pub fn generate_reactive_mask(
        &mut self,
        params: &Fsr3UpscalerGenerateReactiveDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { Fsr3UpscalerContextGenerateReactiveMask(&mut *self.inner, params) };
        ok_or_error(code)
    }

    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, FfxError> {
        let mut usage = EffectMemoryUsage::default();
        ok_or_error(unsafe { Fsr3UpscalerContextGetGpuMemoryUsage(&mut *self.inner, &mut usage) })?;
        Ok(usage)
    }

    pub fn shared_resource_descriptions(
        &mut self,
    ) -> Result<Fsr3UpscalerSharedResourceDescriptions, FfxError> {
        let mut desc = Fsr3UpscalerSharedResourceDescriptions::default();
        ok_or_error(unsafe {
            Fsr3UpscalerGetSharedResourceDescriptions(&mut *self.inner, &mut desc)
        })?;
        Ok(desc)
    }

    pub unsafe fn set_constant(
        &mut self,
        key: Fsr3UpscalerConfigureKey,
        value: *mut std::ffi::c_void,
    ) -> Result<(), FfxError> {
        let code = unsafe { Fsr3UpscalerSetConstant(&mut *self.inner, key, value) };
        ok_or_error(code)
    }

    pub fn set_float_constant(
        &mut self,
        key: Fsr3UpscalerConfigureKey,
        value: f32,
    ) -> Result<(), FfxError> {
        unsafe { self.set_constant(key, &value as *const _ as *mut _) }
    }
}

define_context!(
    Fsr2Context,
    ffi::Fsr2Context,
    Fsr2ContextDescription,
    Fsr2DispatchDescription,
    Fsr2ContextCreate,
    Fsr2ContextDispatch,
    Fsr2ContextDestroy
);

impl Fsr2Context {
    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, FfxError> {
        let mut usage = EffectMemoryUsage::default();
        ok_or_error(unsafe { Fsr2ContextGetGpuMemoryUsage(&mut *self.inner, &mut usage) })?;
        Ok(usage)
    }

    pub fn generate_reactive_mask(
        &mut self,
        desc: &Fsr2GenerateReactiveDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { Fsr2ContextGenerateReactiveMask(&mut *self.inner, desc) };
        ok_or_error(code)
    }
}

define_context!(
    Fsr1Context,
    ffi::Fsr1Context,
    Fsr1ContextDescription,
    Fsr1DispatchDescription,
    Fsr1ContextCreate,
    Fsr1ContextDispatch,
    Fsr1ContextDestroy
);

impl Fsr1Context {
    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, FfxError> {
        let mut usage = EffectMemoryUsage::default();
        ok_or_error(unsafe { Fsr1ContextGetGpuMemoryUsage(&mut *self.inner, &mut usage) })?;
        Ok(usage)
    }
}

define_context!(
    CasContext,
    ffi::CasContext,
    CasContextDescription,
    CasDispatchDescription,
    CasContextCreate,
    CasContextDispatch,
    CasContextDestroy
);

define_context!(
    CacaoContext,
    ffi::CacaoContext,
    CacaoContextDescription,
    CacaoDispatchDescription,
    CacaoContextCreate,
    CacaoContextDispatch,
    CacaoContextDestroy
);

impl CacaoContext {
    pub fn update_settings(
        &mut self,
        settings: &CacaoSettings,
        use_downsampled_ssao: bool,
    ) -> Result<(), FfxError> {
        let code = unsafe { CacaoUpdateSettings(&mut *self.inner, settings, use_downsampled_ssao) };
        ok_or_error(code)
    }
}

define_context!(
    DofContext,
    ffi::DofContext,
    DofContextDescription,
    DofDispatchDescription,
    DofContextCreate,
    DofContextDispatch,
    DofContextDestroy
);

define_context!(
    LensContext,
    ffi::LensContext,
    LensContextDescription,
    LensDispatchDescription,
    LensContextCreate,
    LensContextDispatch,
    LensContextDestroy
);

define_context!(
    LpmContext,
    ffi::LpmContext,
    LpmContextDescription,
    LpmDispatchDescription,
    LpmContextCreate,
    LpmContextDispatch,
    LpmContextDestroy
);

define_context!(
    SpdContext,
    ffi::SpdContext,
    SpdContextDescription,
    SpdDispatchDescription,
    SpdContextCreate,
    SpdContextDispatch,
    SpdContextDestroy
);

define_context!(
    ParallelSortContext,
    ffi::ParallelSortContext,
    ParallelSortContextDescription,
    ParallelSortDispatchDescription,
    ParallelSortContextCreate,
    ParallelSortContextDispatch,
    ParallelSortContextDestroy
);

define_context!(
    SssrContext,
    ffi::SssrContext,
    SssrContextDescription,
    SssrDispatchDescription,
    SssrContextCreate,
    SssrContextDispatch,
    SssrContextDestroy
);

define_context!(
    VrsContext,
    ffi::VrsContext,
    VrsContextDescription,
    VrsDispatchDescription,
    VrsContextCreate,
    VrsContextDispatch,
    VrsContextDestroy
);

define_context!(
    BlurContext,
    ffi::BlurContext,
    BlurContextDescription,
    BlurDispatchDescription,
    BlurContextCreate,
    BlurContextDispatch,
    BlurContextDestroy
);

define_context!(
    ClassifierContext,
    ffi::ClassifierContext,
    ClassifierContextDescription,
    ClassifierShadowDispatchDescription,
    ClassifierContextCreate,
    ClassifierContextShadowDispatch,
    ClassifierContextDestroy
);

impl ClassifierContext {
    pub fn dispatch_reflection(
        &mut self,
        desc: &ClassifierReflectionDispatchDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { ClassifierContextReflectionDispatch(&mut *self.inner, desc) };
        ok_or_error(code)
    }
}

define_context!(
    DenoiserContext,
    ffi::DenoiserContext,
    DenoiserContextDescription,
    DenoiserShadowsDispatchDescription,
    DenoiserContextCreate,
    DenoiserContextDispatchShadows,
    DenoiserContextDestroy
);

impl DenoiserContext {
    pub fn dispatch_reflections(
        &mut self,
        desc: &DenoiserReflectionsDispatchDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { DenoiserContextDispatchReflections(&mut *self.inner, desc) };
        ok_or_error(code)
    }
}

define_context!(
    OpticalFlowContext,
    ffi::OpticalflowContext,
    OpticalflowContextDescription,
    OpticalflowDispatchDescription,
    OpticalflowContextCreate,
    OpticalflowContextDispatch,
    OpticalflowContextDestroy
);

impl OpticalFlowContext {
    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, FfxError> {
        let mut usage = EffectMemoryUsage::default();
        ok_or_error(unsafe { OpticalflowContextGetGpuMemoryUsage(&mut *self.inner, &mut usage) })?;
        Ok(usage)
    }
}

define_context!(
    BrixelizerGIContext,
    ffi::BrixelizerGIContext,
    BrixelizerGIContextDescription,
    BrixelizerGIContextCreate,
    BrixelizerGIContextDestroy
);

impl BrixelizerGIContext {
    pub fn dispatch(
        &mut self,
        desc: &BrixelizerGIDispatchDescription,
        cmd_list: CommandList,
    ) -> Result<(), FfxError> {
        let code = unsafe { BrixelizerGIContextDispatch(&mut *self.inner, desc, cmd_list) };
        ok_or_error(code)
    }

    pub fn debug_visualization(
        &mut self,
        desc: &BrixelizerGIDebugDescription,
        cmd_list: CommandList,
    ) -> Result<(), FfxError> {
        let code =
            unsafe { BrixelizerGIContextDebugVisualization(&mut *self.inner, desc, cmd_list) };
        ok_or_error(code)
    }
}

#[derive(Debug)]
pub struct BrixelizerContext {
    inner: Box<ffi::BrixelizerContext>,
}

impl BrixelizerContext {
    pub fn create(description: &BrixelizerContextDescription) -> Result<Self, FfxError> {
        let mut ctx = unsafe { Box::<ffi::BrixelizerContext>::new_zeroed().assume_init() };
        ok_or_error(unsafe { BrixelizerContextCreate(description, &mut *ctx) })?;
        Ok(Self { inner: ctx })
    }

    pub fn bake_update(
        &mut self,
        desc: &BrixelizerUpdateDescription,
        out_desc: &mut BrixelizerBakedUpdateDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { BrixelizerBakeUpdate(&mut *self.inner, desc, out_desc) };
        ok_or_error(code)
    }

    pub fn update(
        &mut self,
        desc: &mut BrixelizerBakedUpdateDescription,
        scratch_buffer: Resource,
        command_list: CommandList,
    ) -> Result<(), FfxError> {
        let code =
            unsafe { BrixelizerUpdate(&mut *self.inner, desc, scratch_buffer, command_list) };
        ok_or_error(code)
    }

    pub fn get_context_info(
        &mut self,
        context_info: &mut BrixelizerContextInfo,
    ) -> Result<(), FfxError> {
        let code = unsafe { BrixelizerGetContextInfo(&mut *self.inner, context_info) };
        ok_or_error(code)
    }

    pub fn register_buffers(
        &mut self,
        buffer_descs: &[BrixelizerBufferDescription],
    ) -> Result<(), FfxError> {
        let code = unsafe {
            BrixelizerRegisterBuffers(
                &mut *self.inner,
                buffer_descs.as_ptr(),
                buffer_descs.len() as u32,
            )
        };
        ok_or_error(code)
    }

    pub fn unregister_buffers(&mut self, indices: &[u32]) -> Result<(), FfxError> {
        let code = unsafe {
            BrixelizerUnregisterBuffers(&mut *self.inner, indices.as_ptr(), indices.len() as u32)
        };
        ok_or_error(code)
    }

    pub fn create_instances(
        &mut self,
        descs: &[BrixelizerInstanceDescription],
    ) -> Result<(), FfxError> {
        let code = unsafe {
            BrixelizerCreateInstances(&mut *self.inner, descs.as_ptr(), descs.len() as u32)
        };
        ok_or_error(code)
    }

    pub fn delete_instances(
        &mut self,
        instance_ids: &[BrixelizerInstanceID],
    ) -> Result<(), FfxError> {
        let code = unsafe {
            BrixelizerDeleteInstances(
                &mut *self.inner,
                instance_ids.as_ptr(),
                instance_ids.len() as u32,
            )
        };
        ok_or_error(code)
    }

    pub fn get_raw_context(
        &mut self,
        out_context: &mut *mut BrixelizerRawContext,
    ) -> Result<(), FfxError> {
        let code = unsafe { BrixelizerGetRawContext(&mut *self.inner, out_context) };
        ok_or_error(code)
    }
}

impl Drop for BrixelizerContext {
    fn drop(&mut self) {
        unsafe { BrixelizerContextDestroy(&mut *self.inner) };
    }
}

define_context!(
    BreadcrumbsContext,
    ffi::BreadcrumbsContext,
    BreadcrumbsContextDescription,
    BreadcrumbsContextCreate,
    BreadcrumbsContextDestroy
);

impl BreadcrumbsContext {
    pub fn start_frame(&mut self) -> Result<(), FfxError> {
        let code = unsafe { BreadcrumbsStartFrame(&mut *self.inner) };
        ok_or_error(code)
    }

    pub fn register_command_list(
        &mut self,
        desc: &BreadcrumbsCommandListDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { BreadcrumbsRegisterCommandList(&mut *self.inner, desc) };
        ok_or_error(code)
    }

    pub fn register_pipeline(
        &mut self,
        desc: &BreadcrumbsPipelineStateDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { BreadcrumbsRegisterPipeline(&mut *self.inner, desc) };
        ok_or_error(code)
    }

    pub fn set_pipeline(
        &mut self,
        command_list: CommandList,
        pipeline: Pipeline,
    ) -> Result<(), FfxError> {
        let code = unsafe { BreadcrumbsSetPipeline(&mut *self.inner, command_list, pipeline) };
        ok_or_error(code)
    }

    pub fn begin_marker(
        &mut self,
        command_list: CommandList,
        marker_type: BreadcrumbsMarkerType,
        name: &BreadcrumbsNameTag,
    ) -> Result<(), FfxError> {
        let code =
            unsafe { BreadcrumbsBeginMarker(&mut *self.inner, command_list, marker_type, name) };
        ok_or_error(code)
    }

    pub fn end_marker(&mut self, command_list: CommandList) -> Result<(), FfxError> {
        let code = unsafe { BreadcrumbsEndMarker(&mut *self.inner, command_list) };
        ok_or_error(code)
    }

    pub fn print_status(&mut self, status: &mut BreadcrumbsMarkersStatus) -> Result<(), FfxError> {
        let code = unsafe { BreadcrumbsPrintStatus(&mut *self.inner, status) };
        ok_or_error(code)
    }
}

/// A RAII wrapper around an `FfxDevice` obtained from a [`VkDeviceContext`].
///
/// The Vulkan backend stores a single global copy of the `VkDeviceContext`, so
/// creating multiple `Device`s will silently overwrite that global. No
/// destructor is needed — the application owns the Vulkan device lifetime.
pub struct Device {
    raw: ffi::Device,
    physical_device: VkPhysicalDevice,
}

impl Device {
    /// Creates a new device from a [`VkDeviceContext`].
    ///
    /// # Safety
    ///
    /// - `ctx` must contain valid Vulkan handles.
    /// - The backend's global device state will be overwritten.
    pub unsafe fn new(
        device: VkDevice,
        physical_device: VkPhysicalDevice,
    ) -> Result<Self, &'static str> {
        let mut ctx = VkDeviceContext {
            vkDevice: device,
            vkPhysicalDevice: physical_device,
            vkDeviceProcAddr: Some(vkGetDeviceProcAddr),
            instanceFunctions: VkInstanceFunctionTableFFX {
                getPhysicalDeviceFeatures2: Some(vkGetPhysicalDeviceFeatures2),
                enumerateDeviceExtensionProperties: Some(vkEnumerateDeviceExtensionProperties),
                getPhysicalDeviceMemoryProperties: Some(vkGetPhysicalDeviceMemoryProperties),
                getPhysicalDeviceProperties2: Some(vkGetPhysicalDeviceProperties2),
            },
        };
        let raw = unsafe { GetDeviceVK((&mut ctx) as *mut _) };
        if raw.is_null() {
            Err("ffxGetDeviceVK returned null")
        } else {
            Ok(Self {
                raw,
                physical_device: ctx.vkPhysicalDevice,
            })
        }
    }

    pub fn as_raw(&self) -> ffi::Device {
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
    inner: Interface,
    _scratch: Vec<u8>,
}

impl BackendInterface {
    /// Creates a new backend interface.
    ///
    /// # Safety
    ///
    /// `device` must outlive this interface.
    pub unsafe fn new(device: &Device, max_contexts: usize) -> Result<Self, FfxError> {
        let scratch_size = unsafe { GetScratchMemorySizeVK(device.physical_device, max_contexts) };
        let scratch = vec![0u8; scratch_size];
        let mut interface = Interface::default();
        let code = unsafe {
            GetInterfaceVK(
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

    pub fn as_ref(&self) -> &Interface {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut Interface {
        &mut self.inner
    }

    /// Creates an [`Fsr3UpscalerContext`] from this backend interface.
    ///
    /// The [`BackendInterface`] must outlive the returned [`Fsr3UpscalerContext`].
    pub fn create_fsr3_upscaler(
        &self,
        flags: u32,
        render_size: Dimensions2D,
        upscale_size: Dimensions2D,
    ) -> Result<Fsr3UpscalerContext, FfxError> {
        let desc = Fsr3UpscalerContextDescription {
            flags,
            maxRenderSize: render_size,
            maxUpscaleSize: upscale_size,
            fpMessage: None,
            backendInterface: *self.as_ref(),
        };
        Fsr3UpscalerContext::create(&desc)
    }

    /// Creates an [`Fsr2Context`] from this backend interface.
    pub fn create_fsr2(
        &self,
        flags: u32,
        render_size: Dimensions2D,
        display_size: Dimensions2D,
    ) -> Result<Fsr2Context, FfxError> {
        let desc = Fsr2ContextDescription {
            flags,
            maxRenderSize: render_size,
            displaySize: display_size,
            fpMessage: None,
            backendInterface: *self.as_ref(),
        };
        Fsr2Context::create(&desc)
    }

    /// Creates an [`Fsr1Context`] from this backend interface.
    pub fn create_fsr1(
        &self,
        flags: u32,
        output_format: SurfaceFormat,
        render_size: Dimensions2D,
        display_size: Dimensions2D,
    ) -> Result<Fsr1Context, FfxError> {
        let desc = Fsr1ContextDescription {
            flags,
            outputFormat: output_format,
            maxRenderSize: render_size,
            displaySize: display_size,
            backendInterface: *self.as_ref(),
        };
        Fsr1Context::create(&desc)
    }

    /// Creates a [`CasContext`] from this backend interface.
    pub fn create_cas(
        &self,
        flags: u32,
        color_space: CasColorSpaceConversion,
        render_size: Dimensions2D,
        display_size: Dimensions2D,
    ) -> Result<CasContext, FfxError> {
        let desc = CasContextDescription {
            flags,
            colorSpaceConversion: color_space,
            maxRenderSize: render_size,
            displaySize: display_size,
            backendInterface: *self.as_ref(),
        };
        CasContext::create(&desc)
    }

    /// Creates a [`LpmContext`] from this backend interface.
    pub fn create_lpm(&self, flags: u32) -> Result<LpmContext, FfxError> {
        let desc = LpmContextDescription {
            flags,
            backendInterface: *self.as_ref(),
        };
        LpmContext::create(&desc)
    }

    /// Creates a [`SpdContext`] from this backend interface.
    pub fn create_spd(
        &self,
        flags: u32,
        downsample_filter: SpdDownsampleFilter,
    ) -> Result<SpdContext, FfxError> {
        let desc = SpdContextDescription {
            flags,
            downsampleFilter: downsample_filter,
            backendInterface: *self.as_ref(),
        };
        SpdContext::create(&desc)
    }

    /// Creates a [`LensContext`] from this backend interface.
    pub fn create_lens(
        &self,
        flags: u32,
        output_format: SurfaceFormat,
        float_precision: LensFloatPrecision,
    ) -> Result<LensContext, FfxError> {
        let desc = LensContextDescription {
            flags,
            outputFormat: output_format,
            floatPrecision: float_precision,
            backendInterface: *self.as_ref(),
        };
        LensContext::create(&desc)
    }

    /// Creates a [`CacaoContext`] from this backend interface.
    pub fn create_cacao(
        &self,
        width: u32,
        height: u32,
        use_downsampled_ssao: bool,
    ) -> Result<CacaoContext, FfxError> {
        let desc = CacaoContextDescription {
            width,
            height,
            useDownsampledSsao: use_downsampled_ssao,
            backendInterface: *self.as_ref(),
        };
        CacaoContext::create(&desc)
    }

    /// Creates a [`BrixelizerGIContext`] from this backend interface.
    pub fn create_brixelizer_gi(
        &self,
        flags: u32,
        internal_resolution: BrixelizerGIInternalResolution,
        display_size: Dimensions2D,
    ) -> Result<BrixelizerGIContext, FfxError> {
        let desc = BrixelizerGIContextDescription {
            flags,
            internalResolution: internal_resolution,
            displaySize: display_size,
            backendInterface: *self.as_ref(),
        };
        BrixelizerGIContext::create(&desc)
    }

    /// Creates a [`ParallelSortContext`] from this backend interface.
    pub fn create_parallel_sort(
        &self,
        flags: u32,
        max_entries: u32,
    ) -> Result<ParallelSortContext, FfxError> {
        let desc = ParallelSortContextDescription {
            flags,
            maxEntries: max_entries,
            backendInterface: *self.as_ref(),
        };
        ParallelSortContext::create(&desc)
    }

    /// Creates a [`DofContext`] from this backend interface.
    pub fn create_dof(
        &self,
        flags: u32,
        quality: u32,
        resolution: Dimensions2D,
        coc_limit_factor: f32,
    ) -> Result<DofContext, FfxError> {
        let desc = DofContextDescription {
            flags,
            quality,
            resolution,
            cocLimitFactor: coc_limit_factor,
            backendInterface: *self.as_ref(),
        };
        DofContext::create(&desc)
    }

    /// Creates a [`SssrContext`] from this backend interface.
    pub fn create_sssr(
        &self,
        flags: u32,
        render_size: Dimensions2D,
        normals_format: SurfaceFormat,
    ) -> Result<SssrContext, FfxError> {
        let desc = SssrContextDescription {
            flags,
            renderSize: render_size,
            normalsHistoryBufferFormat: normals_format,
            backendInterface: *self.as_ref(),
        };
        SssrContext::create(&desc)
    }

    /// Creates a [`VrsContext`] from this backend interface.
    pub fn create_vrs(
        &self,
        flags: u32,
        shading_rate_tile_size: u32,
    ) -> Result<VrsContext, FfxError> {
        let desc = VrsContextDescription {
            flags,
            shadingRateImageTileSize: shading_rate_tile_size,
            backendInterface: *self.as_ref(),
        };
        VrsContext::create(&desc)
    }

    /// Creates a [`ClassifierContext`] from this backend interface.
    pub fn create_classifier(
        &self,
        flags: u32,
        resolution: Dimensions2D,
    ) -> Result<ClassifierContext, FfxError> {
        let desc = ClassifierContextDescription {
            flags,
            resolution,
            backendInterface: *self.as_ref(),
        };
        ClassifierContext::create(&desc)
    }

    /// Creates a [`DenoiserContext`] from this backend interface.
    pub fn create_denoiser(
        &self,
        flags: u32,
        window_size: Dimensions2D,
        normals_format: SurfaceFormat,
    ) -> Result<DenoiserContext, FfxError> {
        let desc = DenoiserContextDescription {
            flags,
            windowSize: window_size,
            normalsHistoryBufferFormat: normals_format,
            backendInterface: *self.as_ref(),
        };
        DenoiserContext::create(&desc)
    }

    /// Creates an [`OpticalFlowContext`] from this backend interface.
    pub fn create_optical_flow(
        &self,
        flags: u32,
        resolution: Dimensions2D,
    ) -> Result<OpticalFlowContext, FfxError> {
        let desc = OpticalflowContextDescription {
            flags,
            resolution,
            backendInterface: *self.as_ref(),
        };
        OpticalFlowContext::create(&desc)
    }

    /// Creates a [`BlurContext`] from this backend interface.
    pub fn create_blur(
        &self,
        kernel_permutations: BlurKernelPermutations,
        kernel_sizes: BlurKernelSizes,
        float_precision: BlurFloatPrecision,
    ) -> Result<BlurContext, FfxError> {
        let desc = BlurContextDescription {
            kernelPermutations: kernel_permutations,
            kernelSizes: kernel_sizes,
            floatPrecision: float_precision,
            backendInterface: *self.as_ref(),
        };
        BlurContext::create(&desc)
    }
}

#[inline]
pub fn jitter_offset(index: i32, phase_count: i32) -> (f32, f32) {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    unsafe {
        Fsr3UpscalerGetJitterOffset(&mut x, &mut y, index, phase_count);
    }
    (x, y)
}

#[inline]
pub fn render_resolution_from_quality_mode(
    display_width: u32,
    display_height: u32,
    mode: Fsr3UpscalerQualityMode,
) -> Result<(u32, u32), FfxError> {
    let mut w = 0u32;
    let mut h = 0u32;
    ok_or_error(unsafe {
        Fsr3UpscalerGetRenderResolutionFromQualityMode(
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
    fn vkGetDeviceProcAddr(device: VkDevice, pName: *const std::ffi::c_char) -> PFN_vkVoidFunction;
    fn vkGetPhysicalDeviceFeatures2(
        physicalDevice: VkPhysicalDevice,
        pFeatures: *mut VkPhysicalDeviceFeatures2,
    );
    fn vkEnumerateDeviceExtensionProperties(
        physicalDevice: VkPhysicalDevice,
        pLayerName: *const std::ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> VkResult;
    fn vkGetPhysicalDeviceMemoryProperties(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
    );
    fn vkGetPhysicalDeviceProperties2(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties2,
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
        .shader_subgroup_extended_types(true)
        .shader_storage_buffer_array_non_uniform_indexing(true);
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
            .create_fsr3_upscaler(
                Fsr3UpscalerInitializationFlagBits::FFX_FSR3UPSCALER_ENABLE_DEBUG_CHECKING as _,
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

    backend.create_lpm(0).unwrap();

    backend
        .create_spd(0, SpdDownsampleFilter::FFX_SPD_DOWNSAMPLE_FILTER_MEAN)
        .unwrap();

    backend.create_parallel_sort(0, 1024).unwrap();

    backend
        .create_cas(
            0,
            CasColorSpaceConversion::FFX_CAS_COLOR_SPACE_LINEAR,
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

    backend
        .create_lens(
            0,
            SurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
            LensFloatPrecision::FFX_LENS_FLOAT_PRECISION_32BIT,
        )
        .unwrap();

    backend
        .create_fsr2(
            0,
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

    backend
        .create_fsr1(
            0,
            SurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
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

    backend.create_cacao(1280, 720, false).unwrap();

    backend
        .create_dof(
            0,
            5,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
            1.0,
        )
        .unwrap();

    backend
        .create_sssr(
            0,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
            SurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
        )
        .unwrap();

    backend.create_vrs(0, 16).unwrap();

    backend
        .create_classifier(
            ClassifierInitializationFlagBits::FFX_CLASSIFIER_SHADOW as _,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
        )
        .unwrap();

    backend
        .create_denoiser(
            DenoiserInitializationFlagBits::FFX_DENOISER_SHADOWS as _,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
            SurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
        )
        .unwrap();

    backend
        .create_optical_flow(
            0,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
        )
        .unwrap();

    backend
        .create_blur(
            FFX_BLUR_KERNEL_PERMUTATIONS_ALL,
            BlurKernelSize::FFX_BLUR_KERNEL_SIZE_3x3 as u32
                | BlurKernelSize::FFX_BLUR_KERNEL_SIZE_5x5 as u32,
            BlurFloatPrecision::FFX_BLUR_FLOAT_PRECISION_32BIT,
        )
        .unwrap();

    backend
        .create_brixelizer_gi(
            0,
            BrixelizerGIInternalResolution::FFX_BRIXELIZER_GI_INTERNAL_RESOLUTION_NATIVE,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
        )
        .unwrap();

    {
        let alloc_callbacks = AllocationCallbacks {
            fpAlloc: Some(libc::malloc),
            fpRealloc: Some(libc::realloc),
            fpFree: Some(libc::free),
        };
        let queue_type: u32 = 0;
        let _bc = BreadcrumbsContext::create(&BreadcrumbsContextDescription {
            flags: 0,
            frameHistoryLength: 2,
            maxMarkersPerMemoryBlock: 1024,
            usedGpuQueuesCount: 1,
            pUsedGpuQueues: &queue_type as *const _ as *mut _,
            allocCallbacks: alloc_callbacks,
            backendInterface: *backend.as_ref(),
        })
        .unwrap();
    }

    unsafe { device.destroy_device(None) };
    unsafe { instance.destroy_instance(None) };
}
