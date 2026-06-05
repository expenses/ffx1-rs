#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod ffi {
    #![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

pub use ffi::*;

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
    pub fn create(desc: &ffi::FfxFsr3UpscalerContextDescription) -> Result<Self, i32> {
        let mut ctx = Box::new(ffi::FfxFsr3UpscalerContext::default());
        let code = unsafe { ffi::ffxFsr3UpscalerContextCreate(&mut *ctx, desc as *const _) };
        if code == FFX_OK {
            Ok(Self { inner: ctx })
        } else {
            Err(code)
        }
    }

    pub fn dispatch(&mut self, desc: &ffi::FfxFsr3UpscalerDispatchDescription) -> FfxResult {
        let code = unsafe { ffi::ffxFsr3UpscalerContextDispatch(&mut *self.inner, desc) };
        ok_or_error(code)
    }

    pub fn generate_reactive_mask(
        &mut self,
        params: &ffi::FfxFsr3UpscalerGenerateReactiveDescription,
    ) -> FfxResult {
        let code =
            unsafe { ffi::ffxFsr3UpscalerContextGenerateReactiveMask(&mut *self.inner, params) };
        ok_or_error(code)
    }

    pub fn gpu_memory_usage(&mut self) -> Result<ffi::FfxEffectMemoryUsage, i32> {
        let mut usage = ffi::FfxEffectMemoryUsage::default();
        let code =
            unsafe { ffi::ffxFsr3UpscalerContextGetGpuMemoryUsage(&mut *self.inner, &mut usage) };
        if code == FFX_OK { Ok(usage) } else { Err(code) }
    }

    pub fn shared_resource_descriptions(
        &mut self,
    ) -> Result<ffi::FfxFsr3UpscalerSharedResourceDescriptions, i32> {
        let mut desc = ffi::FfxFsr3UpscalerSharedResourceDescriptions::default();
        let code = unsafe {
            ffi::ffxFsr3UpscalerGetSharedResourceDescriptions(&mut *self.inner, &mut desc)
        };
        if code == FFX_OK { Ok(desc) } else { Err(code) }
    }

    pub fn set_constant(
        &mut self,
        key: ffi::FfxFsr3UpscalerConfigureKey,
        value: *mut std::ffi::c_void,
    ) -> FfxResult {
        let code = unsafe { ffi::ffxFsr3UpscalerSetConstant(&mut *self.inner, key, value) };
        ok_or_error(code)
    }

    pub fn set_float_constant(
        &mut self,
        key: ffi::FfxFsr3UpscalerConfigureKey,
        value: f32,
    ) -> FfxResult {
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
        Self {
            resource,
            description,
            state,
            name: [0; 64],
        }
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
pub fn upscale_ratio_from_quality_mode(mode: ffi::FfxFsr3UpscalerQualityMode) -> f32 {
    unsafe { ffi::ffxFsr3UpscalerGetUpscaleRatioFromQualityMode(mode) }
}

#[inline]
pub fn render_resolution_from_quality_mode(
    display_width: u32,
    display_height: u32,
    mode: ffi::FfxFsr3UpscalerQualityMode,
) -> Result<(u32, u32), i32> {
    let mut w = 0u32;
    let mut h = 0u32;
    let code = unsafe {
        ffi::ffxFsr3UpscalerGetRenderResolutionFromQualityMode(
            &mut w,
            &mut h,
            display_width,
            display_height,
            mode,
        )
    };
    if code == FFX_OK {
        Ok((w, h))
    } else {
        Err(code)
    }
}

#[inline]
pub fn is_resource_null(resource: ffi::FfxResource) -> bool {
    unsafe { ffi::ffxFsr3UpscalerResourceIsNull(resource) }
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
        let desc = ffi::FfxFsr3UpscalerDispatchDescription::default();
        assert_eq!(desc.flags, 0);
    }
}
