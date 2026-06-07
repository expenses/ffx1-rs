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

#[derive(Debug)]
pub struct UpscalerContext {
    inner: Box<ffi::FfxFsr3UpscalerContext>,
}

impl UpscalerContext {
    pub fn create(desc: &UpscalerContextDescription) -> Result<Self, FfxError> {
        let mut ctx = Box::new(ffi::FfxFsr3UpscalerContext::default());
        ok_or_error(unsafe { ffi::ffxFsr3UpscalerContextCreate(&mut *ctx, desc as *const _) })?;
        Ok(Self { inner: ctx })
    }

    pub fn dispatch(&mut self, desc: &UpscalerDispatchDescription) -> Result<(), FfxError> {
        let code = unsafe { ffi::ffxFsr3UpscalerContextDispatch(&mut *self.inner, desc) };
        ok_or_error(code)
    }

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

impl Drop for UpscalerContext {
    fn drop(&mut self) {
        unsafe { ffi::ffxFsr3UpscalerContextDestroy(&mut *self.inner) };
    }
}

#[inline]
pub fn jitter_phase_count(render_width: i32, display_width: i32) -> i32 {
    unsafe { ffi::ffxFsr3UpscalerGetJitterPhaseCount(render_width, display_width) }
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
pub fn upscale_ratio_from_quality_mode(mode: UpscalerQualityMode) -> f32 {
    unsafe { ffi::ffxFsr3UpscalerGetUpscaleRatioFromQualityMode(mode) }
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

#[inline]
pub fn is_resource_null(resource: ffi::FfxResource) -> bool {
    unsafe { ffi::ffxFsr3UpscalerResourceIsNull(resource) }
}

pub unsafe fn scratch_memory_size(
    physical_device: ffi::VkPhysicalDevice,
    max_contexts: usize,
) -> usize {
    unsafe { ffi::ffxGetScratchMemorySizeVK(physical_device, max_contexts) }
}

pub unsafe fn get_device(ctx: *mut ffi::VkDeviceContext) -> ffi::FfxDevice {
    unsafe { ffi::ffxGetDeviceVK(ctx) }
}

pub unsafe fn get_interface(
    backend_interface: *mut ffi::FfxInterface,
    device: ffi::FfxDevice,
    scratch_buffer: *mut std::ffi::c_void,
    scratch_buffer_size: usize,
    max_contexts: usize,
) -> i32 {
    unsafe {
        ffi::ffxGetInterfaceVK(
            backend_interface,
            device,
            scratch_buffer,
            scratch_buffer_size,
            max_contexts,
        )
    }
}

pub unsafe fn get_command_list(cmd_buf: ffi::VkCommandBuffer) -> ffi::FfxCommandList {
    unsafe { ffi::ffxGetCommandListVK(cmd_buf) }
}

pub unsafe fn get_resource(
    vk_resource: *mut std::ffi::c_void,
    description: ffi::FfxResourceDescription,
    name: *const u32,
    state: u32,
) -> ffi::FfxResource {
    unsafe { ffi::ffxGetResourceVK(vk_resource, description, name, state) }
}

// pub fn create_interface(
//     physical_device: ffi::VkPhysicalDevice,
//     vk_device: ffi::VkDevice,
//     device_proc_addr: ffi::PFN_vkGetDeviceProcAddr,
//     instance_table: ffi::VkInstanceFunctionTableFFX,
//     max_contexts: usize,
// ) -> Result<(ffi::FfxInterface, Vec<u8>), super::FfxError> {
//     let scratch_size = scratch_memory_size(physical_device, max_contexts);
//     let mut scratch = vec![0u8; scratch_size];

//     let mut ctx = ffi::VkDeviceContext {
//         vkDevice: vk_device,
//         vkPhysicalDevice: physical_device,
//         vkDeviceProcAddr: device_proc_addr,
//         instanceFunctions: instance_table,
//     };

//     let device = unsafe { get_device(&mut ctx) };
//     if device.is_null() {
//         return Err(super::FfxError(ffi::FfxErrorCodes::FFX_ERROR_INVALID_POINTER as i32));
//     }

//     let mut interface = ffi::FfxInterface::default();
//     let code = unsafe {
//         get_interface(&mut interface, device, scratch.as_mut_ptr() as *mut _, scratch_size, max_contexts)
//     };
//     if code != super::FFX_OK { Err(super::FfxError(code)) } else { Ok((interface, scratch)) }
// }

// pub fn create_upscaler(
//     physical_device: ffi::VkPhysicalDevice,
//     vk_device: ffi::VkDevice,
//     device_proc_addr: ffi::PFN_vkGetDeviceProcAddr,
//     instance_table: ffi::VkInstanceFunctionTableFFX,
//     flags: u32,
//     render_size: super::Dimensions2D,
//     upscale_size: super::Dimensions2D,
// ) -> Result<(super::UpscalerContext, Vec<u8>), super::FfxError> {
//     let (interface, scratch) = create_interface(physical_device, vk_device, device_proc_addr, instance_table, 1)?;
//     let desc = super::UpscalerContextDescription {
//         flags,
//         maxRenderSize: render_size,
//         maxUpscaleSize: upscale_size,
//         fpMessage: None,
//         backendInterface: interface,
//     };
//     let ctx = super::UpscalerContext::create(&desc)?;
//     Ok((ctx, scratch))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdk_version() {
        assert_eq!(ffi::FFX_FSR3UPSCALER_VERSION_MAJOR, 3);
        assert_eq!(ffi::FFX_FSR3UPSCALER_VERSION_MINOR, 1);
        assert_eq!(ffi::FFX_FSR3UPSCALER_VERSION_PATCH, 4);
    }

    #[test]
    fn jitter_helpers() {
        let count = jitter_phase_count(1920, 3840);
        assert!(count > 0);
        let (x, y) = jitter_offset(0, count);
        assert!(x != 0.0 || y != 0.0);
    }

    #[test]
    fn resource_construction() {
        let r = ffi::FfxResource::default();
        assert!(r.resource.is_null());
    }

    #[test]
    fn dispatch_default_allows_zero() {
        let desc = UpscalerDispatchDescription::default();
        assert_eq!(desc.flags, 0);
    }

    #[test]
    fn type_aliases_work() {
        let _ = UpscalerContextDescription {
            flags: 0,
            maxRenderSize: Dimensions2D {
                width: 1920,
                height: 1080,
            },
            maxUpscaleSize: Dimensions2D {
                width: 1920,
                height: 1080,
            },
            fpMessage: None,
            backendInterface: unsafe { std::mem::zeroed() },
        };
    }

    #[test]
    fn vk_types_exist() {
        let _ = ffi::VkDeviceContext::default();
        let _ = ffi::VkInstanceFunctionTableFFX::default();
    }
}
