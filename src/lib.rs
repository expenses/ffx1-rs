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
    // Boxed because the opaque context struct is ~838 KB on x64
    // (FFX_SDK_DEFAULT_CONTEXT_SIZE = 837848 / 4 u32s).
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
        let mut scratch = vec![0u8; scratch_size];
        let mut interface = ffi::FfxInterface::default();
        let code = unsafe {
            ffi::ffxGetInterfaceVK(
                &mut interface,
                device.raw,
                scratch.as_mut_ptr() as *mut _,
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

    #[test]
    fn raii_types_compile() {
        let _ = ffi::FfxDevice::default();
        let _ = ffi::FfxInterface::default();
    }

    #[test]
    fn device_creation_with_ash() {
        use ash::vk::Handle;

        let entry = unsafe { ash::Entry::load() }.expect("failed to load vulkan");

        let app_info = ash::vk::ApplicationInfo::default().api_version(ash::vk::API_VERSION_1_0);
        let create_info = ash::vk::InstanceCreateInfo::default().application_info(&app_info);
        let instance = unsafe { entry.create_instance(&create_info, None) }
            .expect("failed to create instance");

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
        let device_create_info =
            ash::vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);
        let device = unsafe { instance.create_device(physical_device, &device_create_info, None) }
            .expect("create_device");

        let ffx_device =
            unsafe { Device::new(device.handle().as_raw() as _, physical_device.as_raw() as _) }
                .expect("Device::new failed");
        assert!(!ffx_device.as_raw().is_null());

        let _backend = unsafe { BackendInterface::new(&ffx_device, 1) }.unwrap();

        unsafe { device.destroy_device(None) };
        unsafe { instance.destroy_instance(None) };
    }
}
