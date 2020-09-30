
    use ffi::*;
    use ffi::AVHWDeviceType::*;
    use std::ffi::CString;

    #[derive(Eq, PartialEq, Clone, Copy, Debug)]
    pub enum HWDeviceType {
        None,
        VDPAU,
        CUDA,
        VAAPI,
        DXVA2,
        QSV,
        VIDEOTOOLBOX,
        D3D11VA,
        DRM,
        OPENCL,
        MEDIACODEC,
        VULKAN,         // add feature 4.3
    }

    impl From<AVHWDeviceType> for HWDeviceType {
        #[inline(always)]
        fn from(value: AVHWDeviceType) -> Self {
            match value {
                AV_HWDEVICE_TYPE_NONE => HWDeviceType::None,
                AV_HWDEVICE_TYPE_VDPAU => HWDeviceType::VDPAU,
                AV_HWDEVICE_TYPE_CUDA => HWDeviceType::CUDA,
                AV_HWDEVICE_TYPE_VAAPI => HWDeviceType::VAAPI,
                AV_HWDEVICE_TYPE_DXVA2 => HWDeviceType::DXVA2,
                AV_HWDEVICE_TYPE_QSV => HWDeviceType::QSV,
                AV_HWDEVICE_TYPE_VIDEOTOOLBOX => HWDeviceType::VIDEOTOOLBOX,
                AV_HWDEVICE_TYPE_D3D11VA => HWDeviceType::D3D11VA,
                AV_HWDEVICE_TYPE_DRM => HWDeviceType::DRM,
                AV_HWDEVICE_TYPE_OPENCL => HWDeviceType::OPENCL,
                AV_HWDEVICE_TYPE_MEDIACODEC => HWDeviceType::MEDIACODEC,
                AV_HWDEVICE_TYPE_VULKAN => HWDeviceType::VULKAN,
            }
        }
    }

    impl Into<AVHWDeviceType> for HWDeviceType {
        #[inline(always)]
        fn into(self) -> AVHWDeviceType {
            match self {
                HWDeviceType::None => AV_HWDEVICE_TYPE_NONE,
                HWDeviceType::VDPAU => AV_HWDEVICE_TYPE_VDPAU,
                HWDeviceType::CUDA => AV_HWDEVICE_TYPE_CUDA,
                HWDeviceType::VAAPI => AV_HWDEVICE_TYPE_VAAPI,
                HWDeviceType::DXVA2 => AV_HWDEVICE_TYPE_DXVA2,
                HWDeviceType::QSV => AV_HWDEVICE_TYPE_QSV,
                HWDeviceType::VIDEOTOOLBOX => AV_HWDEVICE_TYPE_VIDEOTOOLBOX,
                HWDeviceType::D3D11VA => AV_HWDEVICE_TYPE_D3D11VA,
                HWDeviceType::DRM => AV_HWDEVICE_TYPE_DRM,
                HWDeviceType::OPENCL => AV_HWDEVICE_TYPE_OPENCL,
                HWDeviceType::MEDIACODEC => AV_HWDEVICE_TYPE_MEDIACODEC,
                HWDeviceType::VULKAN => AV_HWDEVICE_TYPE_VULKAN,
            }
        }
    }

    pub fn find_type_by_name(name: &str) -> Option<HWDeviceType> {
        unsafe {
            let name = CString::new(name).unwrap();
            let t = HWDeviceType::from(av_hwdevice_find_type_by_name(name.as_ptr()));
            if t != HWDeviceType::None {
                Some(t)
            }
            else {
                None
            }
        }
    }

    pub fn type_name(type_: HWDeviceType) -> &'static str {
        unsafe {
            let c_str = av_hwdevice_get_type_name(type_.into());
            std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(c_str).to_bytes())
        }
    }

    pub struct TypeIterator {
        prev_type: HWDeviceType,
    }
    
    impl TypeIterator {
        fn new() -> Self {
            TypeIterator {prev_type: HWDeviceType::None}
        }
    }
    
    impl Iterator for TypeIterator {
        type Item = HWDeviceType;
    
        fn next(&mut self) -> Option<Self::Item> {
            self.prev_type = unsafe { HWDeviceType::from(av_hwdevice_iterate_types(self.prev_type.into())) };
            match self.prev_type {
                HWDeviceType::None => None,
                t => Some(t),
            }
        }
    }

    pub fn hw_device_types() -> TypeIterator {
        TypeIterator::new()
    }