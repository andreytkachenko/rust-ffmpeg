use ffi::*;
use super::hw_device_type::HWDeviceType;
use format::pixel::Pixel;

use std::os::raw::c_int;
use std::fmt;

pub struct CodecHWConfig {
    ptr: *const AVCodecHWConfig,
}

impl CodecHWConfig {
    pub unsafe  fn wrap(ptr: *const AVCodecHWConfig) -> Self {
        CodecHWConfig {ptr}
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodecHWConfig {
        self.ptr
    }
}

impl CodecHWConfig {
    pub fn device_type(&self) -> HWDeviceType {
        unsafe {
            let dt = (*self.ptr).device_type;
            HWDeviceType::from(dt)
        }
    }

    pub fn methods(&self) -> MethodFlags{
        unsafe {
            let methods = (*self.ptr).methods;
            MethodFlags::from_bits_truncate(methods)
        }
    }

    pub fn pix_fmt(&self) -> Pixel {
        unsafe {
            let pix_fmt = (*self.ptr).pix_fmt;
            Pixel::from(pix_fmt)
        }
    }
}

impl fmt::Debug for CodecHWConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodecHWConfig")
            .field("device_type", &self.device_type())
            .field("methods", &self.methods())
            .field("pix_fmt", &self.pix_fmt())
            .finish()
    }
}

bitflags! {
    pub struct MethodFlags: c_int {
        const HW_DEVICE_CTX = 0x1; //AV_CODEC_HW_CONFIG_METHOD_HW_DEVICE_CTX;
        const HW_FRAMES_CTX = 0x2; //AV_CODEC_HW_CONFIG_METHOD_HW_FRAMES_CTX;
        const INTERNAL = 0x4; //AV_CODEC_HW_CONFIG_METHOD_INTERNAL;
        const AD_HOC = 0x8; //AV_CODEC_HW_CONFIG_METHOD_AD_HOC;
    }
}