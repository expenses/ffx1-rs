#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod ffi {
    #![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
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

pub type FfxResult = Result<(), i32>;

#[inline]
pub fn ok_or_error(code: i32) -> FfxResult {
    if code == FFX_OK { Ok(()) } else { Err(code) }
}

#[derive(Debug)]
pub struct UpscalerContext {
    inner: Box<ffi::FfxFsr3UpscalerContext>,
}

impl UpscalerContext {
    pub fn create(desc: &UpscalerContextDescription) -> Result<Self, i32> {
        let mut ctx = Box::new(ffi::FfxFsr3UpscalerContext::default());
        let code = unsafe { ffi::ffxFsr3UpscalerContextCreate(&mut *ctx, desc as *const _) };
        if code == FFX_OK {
            Ok(Self { inner: ctx })
        } else {
            Err(code)
        }
    }

    pub fn dispatch(&mut self, desc: &UpscalerDispatchDescription) -> FfxResult {
        let code = unsafe { ffi::ffxFsr3UpscalerContextDispatch(&mut *self.inner, desc) };
        ok_or_error(code)
    }

    pub fn generate_reactive_mask(&mut self, params: &UpscalerGenerateReactiveDescription) -> FfxResult {
        let code = unsafe { ffi::ffxFsr3UpscalerContextGenerateReactiveMask(&mut *self.inner, params) };
        ok_or_error(code)
    }

    pub fn gpu_memory_usage(&mut self) -> Result<EffectMemoryUsage, i32> {
        let mut usage = ffi::FfxEffectMemoryUsage::default();
        let code = unsafe { ffi::ffxFsr3UpscalerContextGetGpuMemoryUsage(&mut *self.inner, &mut usage) };
        if code == FFX_OK { Ok(usage) } else { Err(code) }
    }

    pub fn shared_resource_descriptions(&mut self) -> Result<UpscalerSharedResourceDescriptions, i32> {
        let mut desc = UpscalerSharedResourceDescriptions::default();
        let code = unsafe { ffi::ffxFsr3UpscalerGetSharedResourceDescriptions(&mut *self.inner, &mut desc) };
        if code == FFX_OK { Ok(desc) } else { Err(code) }
    }

    pub fn set_constant(&mut self, key: UpscalerConfigureKey, value: *mut std::ffi::c_void) -> FfxResult {
        let code = unsafe { ffi::ffxFsr3UpscalerSetConstant(&mut *self.inner, key, value) };
        ok_or_error(code)
    }

    pub fn set_float_constant(&mut self, key: UpscalerConfigureKey, value: f32) -> FfxResult {
        self.set_constant(key, &value as *const _ as *mut _)
    }
}

impl Drop for UpscalerContext {
    fn drop(&mut self) {
        unsafe { ffi::ffxFsr3UpscalerContextDestroy(&mut *self.inner) };
    }
}

impl ffi::FfxResource {
    pub fn null() -> Self {
        Self {
            resource: std::ptr::null_mut(),
            description: ffi::FfxResourceDescription::default(),
            state: 0,
            name: [0; 64],
        }
    }

    pub fn from_raw(
        resource: *mut std::ffi::c_void,
        description: ffi::FfxResourceDescription,
        state: u32,
    ) -> Self {
        Self { resource, description, state, name: [0; 64] }
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
    unsafe { ffi::ffxFsr3UpscalerGetJitterOffset(&mut x, &mut y, index, phase_count); }
    (x, y)
}

#[inline]
pub fn upscale_ratio_from_quality_mode(mode: UpscalerQualityMode) -> f32 {
    unsafe { ffi::ffxFsr3UpscalerGetUpscaleRatioFromQualityMode(mode) }
}

#[inline]
pub fn render_resolution_from_quality_mode(
    display_width: u32, display_height: u32, mode: UpscalerQualityMode,
) -> Result<(u32, u32), i32> {
    let mut w = 0u32;
    let mut h = 0u32;
    let code = unsafe {
        ffi::ffxFsr3UpscalerGetRenderResolutionFromQualityMode(&mut w, &mut h, display_width, display_height, mode)
    };
    if code == FFX_OK { Ok((w, h)) } else { Err(code) }
}

#[inline]
pub fn is_resource_null(resource: ffi::FfxResource) -> bool {
    unsafe { ffi::ffxFsr3UpscalerResourceIsNull(resource) }
}

pub mod vk {
    use super::ffi;

    pub fn scratch_memory_size(physical_device: ffi::VkPhysicalDevice, max_contexts: usize) -> usize {
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
        unsafe { ffi::ffxGetInterfaceVK(backend_interface, device, scratch_buffer, scratch_buffer_size, max_contexts) }
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

    pub fn create_interface(
        physical_device: ffi::VkPhysicalDevice,
        vk_device: ffi::VkDevice,
        device_proc_addr: ffi::PFN_vkGetDeviceProcAddr,
        instance_table: ffi::VkInstanceFunctionTableFFX,
        max_contexts: usize,
    ) -> Result<(ffi::FfxInterface, Vec<u8>), i32> {
        let scratch_size = scratch_memory_size(physical_device, max_contexts);
        let mut scratch = vec![0u8; scratch_size];

        let mut ctx = ffi::VkDeviceContext {
            vkDevice: vk_device,
            vkPhysicalDevice: physical_device,
            vkDeviceProcAddr: device_proc_addr,
            instanceFunctions: instance_table,
        };

        let device = unsafe { get_device(&mut ctx) };
        if device.is_null() {
            return Err(ffi::FfxErrorCodes::FFX_ERROR_INVALID_POINTER as i32);
        }

        let mut interface = ffi::FfxInterface::default();
        let code = unsafe {
            get_interface(&mut interface, device, scratch.as_mut_ptr() as *mut _, scratch_size, max_contexts)
        };
        if code != super::FFX_OK { Err(code) } else { Ok((interface, scratch)) }
    }

    pub fn create_upscaler(
        physical_device: ffi::VkPhysicalDevice,
        vk_device: ffi::VkDevice,
        device_proc_addr: ffi::PFN_vkGetDeviceProcAddr,
        instance_table: ffi::VkInstanceFunctionTableFFX,
        flags: u32,
        render_size: super::Dimensions2D,
        upscale_size: super::Dimensions2D,
    ) -> Result<(super::UpscalerContext, Vec<u8>), i32> {
        let (interface, scratch) = create_interface(physical_device, vk_device, device_proc_addr, instance_table, 1)?;
        let desc = super::UpscalerContextDescription {
            flags,
            maxRenderSize: render_size,
            maxUpscaleSize: upscale_size,
            fpMessage: None,
            backendInterface: interface,
        };
        let ctx = super::UpscalerContext::create(&desc)?;
        Ok((ctx, scratch))
    }
}

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
        let r = ffi::FfxResource::null();
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
            maxRenderSize: Dimensions2D { width: 1920, height: 1080 },
            maxUpscaleSize: Dimensions2D { width: 1920, height: 1080 },
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
