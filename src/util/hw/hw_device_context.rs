use ffi::*;
use super::hw_device_type::HWDeviceType;
use Error;

use std::ptr::{null, null_mut};

pub struct HWDeviceContext {
    ctx_ref: *mut AVBufferRef,
}

impl HWDeviceContext {
    pub unsafe fn wrap(ctx_ref: *mut AVBufferRef) -> Self {
        Self { ctx_ref }
    }

    pub unsafe fn as_ptr(&self) -> *const AVBufferRef {
        self.ctx_ref as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVBufferRef {
        self.ctx_ref
    }

    pub unsafe fn raw_ref(&self) -> Result<*mut AVBufferRef, Error> {
        let ctx_ref = av_buffer_ref(self.as_ptr() as *mut _);
        if !ctx_ref.is_null() {
            Ok(ctx_ref)
        }
        else {
            Err(Error::Other{errno: ENOMEM})
        }
    }
}

impl Drop for HWDeviceContext {
    fn drop(&mut self) {
        unsafe {
            av_buffer_unref(&mut self.as_mut_ptr());
        }
    }
}

impl Clone for HWDeviceContext {
    fn clone_from(&mut self, source: &Self) {
        unsafe {
            av_buffer_unref(&mut self.as_mut_ptr());
            self.ctx_ref = source.raw_ref().unwrap();
        }

    }

    fn clone(&self) -> Self {
        unsafe { Self {ctx_ref: self.raw_ref().unwrap()} }
    }
}

impl HWDeviceContext {
    pub fn create(device_type: HWDeviceType/*, options: &mut Dictionary*/) -> Result<Self, Error> {
        let mut ctx_ref = null_mut();
        let ret = unsafe { av_hwdevice_ctx_create(&mut ctx_ref, device_type.into(), null(), null_mut(), 0) };
        match ret {
            0 => Ok(Self {ctx_ref}),
            e => Err(Error::from(e)),
        }
    }
}


