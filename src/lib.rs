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
    ffi::FfxFsr3UpscalerContext,
    UpscalerContextDescription,
    UpscalerDispatchDescription,
    ffi::ffxFsr3UpscalerContextCreate,
    ffi::ffxFsr3UpscalerContextDispatch,
    ffi::ffxFsr3UpscalerContextDestroy
);

impl Fsr3UpscalerContext {
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

define_context!(
    BrixelizerGIContext,
    ffi::FfxBrixelizerGIContext,
    ffi::FfxBrixelizerGIContextDescription,
    ffi::ffxBrixelizerGIContextCreate,
    ffi::ffxBrixelizerGIContextDestroy
);

impl BrixelizerGIContext {
    pub fn dispatch(
        &mut self,
        desc: &ffi::FfxBrixelizerGIDispatchDescription,
        cmd_list: ffi::FfxCommandList,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBrixelizerGIContextDispatch(&mut *self.inner, desc, cmd_list) };
        ok_or_error(code)
    }

    pub fn debug_visualization(
        &mut self,
        desc: &ffi::FfxBrixelizerGIDebugDescription,
        cmd_list: ffi::FfxCommandList,
    ) -> Result<(), FfxError> {
        let code = unsafe {
            ffi::ffxBrixelizerGIContextDebugVisualization(&mut *self.inner, desc, cmd_list)
        };
        ok_or_error(code)
    }
}

#[derive(Debug)]
pub struct BrixelizerContext {
    inner: Box<ffi::FfxBrixelizerContext>,
}

impl BrixelizerContext {
    pub fn create(description: &ffi::FfxBrixelizerContextDescription) -> Result<Self, FfxError> {
        let mut ctx = unsafe { Box::<ffi::FfxBrixelizerContext>::new_zeroed().assume_init() };
        ok_or_error(unsafe { ffi::ffxBrixelizerContextCreate(description, &mut *ctx) })?;
        Ok(Self { inner: ctx })
    }

    pub fn bake_update(
        &mut self,
        desc: &ffi::FfxBrixelizerUpdateDescription,
        out_desc: &mut ffi::FfxBrixelizerBakedUpdateDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBrixelizerBakeUpdate(&mut *self.inner, desc, out_desc) };
        ok_or_error(code)
    }

    pub fn update(
        &mut self,
        desc: &mut ffi::FfxBrixelizerBakedUpdateDescription,
        scratch_buffer: ffi::FfxResource,
        command_list: ffi::FfxCommandList,
    ) -> Result<(), FfxError> {
        let code = unsafe {
            ffi::ffxBrixelizerUpdate(&mut *self.inner, desc, scratch_buffer, command_list)
        };
        ok_or_error(code)
    }

    pub fn get_context_info(
        &mut self,
        context_info: &mut ffi::FfxBrixelizerContextInfo,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBrixelizerGetContextInfo(&mut *self.inner, context_info) };
        ok_or_error(code)
    }

    pub fn register_buffers(
        &mut self,
        buffer_descs: &[ffi::FfxBrixelizerBufferDescription],
    ) -> Result<(), FfxError> {
        let code = unsafe {
            ffi::ffxBrixelizerRegisterBuffers(
                &mut *self.inner,
                buffer_descs.as_ptr(),
                buffer_descs.len() as u32,
            )
        };
        ok_or_error(code)
    }

    pub fn unregister_buffers(&mut self, indices: &[u32]) -> Result<(), FfxError> {
        let code = unsafe {
            ffi::ffxBrixelizerUnregisterBuffers(
                &mut *self.inner,
                indices.as_ptr(),
                indices.len() as u32,
            )
        };
        ok_or_error(code)
    }

    pub fn create_instances(
        &mut self,
        descs: &[ffi::FfxBrixelizerInstanceDescription],
    ) -> Result<(), FfxError> {
        let code = unsafe {
            ffi::ffxBrixelizerCreateInstances(&mut *self.inner, descs.as_ptr(), descs.len() as u32)
        };
        ok_or_error(code)
    }

    pub fn delete_instances(
        &mut self,
        instance_ids: &[ffi::FfxBrixelizerInstanceID],
    ) -> Result<(), FfxError> {
        let code = unsafe {
            ffi::ffxBrixelizerDeleteInstances(
                &mut *self.inner,
                instance_ids.as_ptr(),
                instance_ids.len() as u32,
            )
        };
        ok_or_error(code)
    }

    pub fn get_raw_context(
        &mut self,
        out_context: &mut *mut ffi::FfxBrixelizerRawContext,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBrixelizerGetRawContext(&mut *self.inner, out_context) };
        ok_or_error(code)
    }
}

impl Drop for BrixelizerContext {
    fn drop(&mut self) {
        unsafe { ffi::ffxBrixelizerContextDestroy(&mut *self.inner) };
    }
}

define_context!(
    BreadcrumbsContext,
    ffi::FfxBreadcrumbsContext,
    ffi::FfxBreadcrumbsContextDescription,
    ffi::ffxBreadcrumbsContextCreate,
    ffi::ffxBreadcrumbsContextDestroy
);

impl BreadcrumbsContext {
    pub fn start_frame(&mut self) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBreadcrumbsStartFrame(&mut *self.inner) };
        ok_or_error(code)
    }

    pub fn register_command_list(
        &mut self,
        desc: &ffi::FfxBreadcrumbsCommandListDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBreadcrumbsRegisterCommandList(&mut *self.inner, desc) };
        ok_or_error(code)
    }

    pub fn register_pipeline(
        &mut self,
        desc: &ffi::FfxBreadcrumbsPipelineStateDescription,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBreadcrumbsRegisterPipeline(&mut *self.inner, desc) };
        ok_or_error(code)
    }

    pub fn set_pipeline(
        &mut self,
        command_list: ffi::FfxCommandList,
        pipeline: ffi::FfxPipeline,
    ) -> Result<(), FfxError> {
        let code =
            unsafe { ffi::ffxBreadcrumbsSetPipeline(&mut *self.inner, command_list, pipeline) };
        ok_or_error(code)
    }

    pub fn begin_marker(
        &mut self,
        command_list: ffi::FfxCommandList,
        marker_type: ffi::FfxBreadcrumbsMarkerType,
        name: &ffi::FfxBreadcrumbsNameTag,
    ) -> Result<(), FfxError> {
        let code = unsafe {
            ffi::ffxBreadcrumbsBeginMarker(&mut *self.inner, command_list, marker_type, name)
        };
        ok_or_error(code)
    }

    pub fn end_marker(&mut self, command_list: ffi::FfxCommandList) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBreadcrumbsEndMarker(&mut *self.inner, command_list) };
        ok_or_error(code)
    }

    pub fn print_status(
        &mut self,
        status: &mut ffi::FfxBreadcrumbsMarkersStatus,
    ) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxBreadcrumbsPrintStatus(&mut *self.inner, status) };
        ok_or_error(code)
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

    /// Creates an [`Fsr3UpscalerContext`] from this backend interface.
    ///
    /// The [`BackendInterface`] must outlive the returned [`Fsr3UpscalerContext`].
    pub fn create_fsr3_upscaler(
        &self,
        flags: u32,
        render_size: Dimensions2D,
        upscale_size: Dimensions2D,
    ) -> Result<Fsr3UpscalerContext, FfxError> {
        let desc = UpscalerContextDescription {
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
        let desc = ffi::FfxFsr2ContextDescription {
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
        output_format: ffi::FfxSurfaceFormat,
        render_size: Dimensions2D,
        display_size: Dimensions2D,
    ) -> Result<Fsr1Context, FfxError> {
        let desc = ffi::FfxFsr1ContextDescription {
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
        color_space: ffi::FfxCasColorSpaceConversion,
        render_size: Dimensions2D,
        display_size: Dimensions2D,
    ) -> Result<CasContext, FfxError> {
        let desc = ffi::FfxCasContextDescription {
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
        let desc = ffi::FfxLpmContextDescription {
            flags,
            backendInterface: *self.as_ref(),
        };
        LpmContext::create(&desc)
    }

    /// Creates a [`SpdContext`] from this backend interface.
    pub fn create_spd(
        &self,
        flags: u32,
        downsample_filter: ffi::FfxSpdDownsampleFilter,
    ) -> Result<SpdContext, FfxError> {
        let desc = ffi::FfxSpdContextDescription {
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
        output_format: ffi::FfxSurfaceFormat,
        float_precision: ffi::FfxLensFloatPrecision,
    ) -> Result<LensContext, FfxError> {
        let desc = ffi::FfxLensContextDescription {
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
        let desc = ffi::FfxCacaoContextDescription {
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
        flags: ffi::FfxBrixelizerGIFlags::Type,
        internal_resolution: ffi::FfxBrixelizerGIInternalResolution,
        display_size: Dimensions2D,
    ) -> Result<BrixelizerGIContext, FfxError> {
        let desc = ffi::FfxBrixelizerGIContextDescription {
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
        let desc = ffi::FfxParallelSortContextDescription {
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
        let desc = ffi::FfxDofContextDescription {
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
        normals_format: ffi::FfxSurfaceFormat,
    ) -> Result<SssrContext, FfxError> {
        let desc = ffi::FfxSssrContextDescription {
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
        let desc = ffi::FfxVrsContextDescription {
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
        let desc = ffi::FfxClassifierContextDescription {
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
        normals_format: ffi::FfxSurfaceFormat,
    ) -> Result<DenoiserContext, FfxError> {
        let desc = ffi::FfxDenoiserContextDescription {
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
        let desc = ffi::FfxOpticalflowContextDescription {
            flags,
            resolution,
            backendInterface: *self.as_ref(),
        };
        OpticalFlowContext::create(&desc)
    }

    /// Creates a [`BlurContext`] from this backend interface.
    pub fn create_blur(
        &self,
        kernel_permutations: ffi::FfxBlurKernelPermutations,
        kernel_sizes: ffi::FfxBlurKernelSizes,
        float_precision: ffi::FfxBlurFloatPrecision,
    ) -> Result<BlurContext, FfxError> {
        let desc = ffi::FfxBlurContextDescription {
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

    backend.create_lpm(0).unwrap();

    backend
        .create_spd(
            0,
            ffi::FfxSpdDownsampleFilter::FFX_SPD_DOWNSAMPLE_FILTER_MEAN,
        )
        .unwrap();

    backend.create_parallel_sort(0, 1024).unwrap();

    backend
        .create_cas(
            0,
            ffi::FfxCasColorSpaceConversion::FFX_CAS_COLOR_SPACE_LINEAR,
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
            ffi::FfxSurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
            ffi::FfxLensFloatPrecision::FFX_LENS_FLOAT_PRECISION_32BIT,
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
            ffi::FfxSurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
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
            ffi::FfxSurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
        )
        .unwrap();

    backend.create_vrs(0, 16).unwrap();

    backend
        .create_classifier(
            ffi::FfxClassifierInitializationFlagBits::FFX_CLASSIFIER_SHADOW,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
        )
        .unwrap();

    backend
        .create_denoiser(
            ffi::FfxDenoiserInitializationFlagBits::FFX_DENOISER_SHADOWS,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
            ffi::FfxSurfaceFormat::FFX_SURFACE_FORMAT_R16G16B16A16_FLOAT,
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
            ffi::FFX_BLUR_KERNEL_PERMUTATIONS_ALL,
            ffi::FfxBlurKernelSize::FFX_BLUR_KERNEL_SIZE_3x3 as u32
                | ffi::FfxBlurKernelSize::FFX_BLUR_KERNEL_SIZE_5x5 as u32,
            ffi::FfxBlurFloatPrecision::FFX_BLUR_FLOAT_PRECISION_32BIT,
        )
        .unwrap();

    backend
        .create_brixelizer_gi(
            0,
            ffi::FfxBrixelizerGIInternalResolution::FFX_BRIXELIZER_GI_INTERNAL_RESOLUTION_NATIVE,
            Dimensions2D {
                width: 1280,
                height: 720,
            },
        )
        .unwrap();

    {
        let alloc_callbacks = ffi::FfxAllocationCallbacks {
            fpAlloc: Some(libc::malloc),
            fpRealloc: Some(libc::realloc),
            fpFree: Some(libc::free),
        };
        let queue_type: u32 = 0;
        let _bc = BreadcrumbsContext::create(&ffi::FfxBreadcrumbsContextDescription {
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
