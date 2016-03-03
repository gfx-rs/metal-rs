#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLBlendFactor {
    MTLBlendFactorZero = 0,
    MTLBlendFactorOne = 1,
    MTLBlendFactorSourceColor = 2,
    MTLBlendFactorOneMinusSourceColor = 3,
    MTLBlendFactorSourceAlpha = 4,
    MTLBlendFactorOneMinusSourceAlpha = 5,
    MTLBlendFactorDestinationColor = 6,
    MTLBlendFactorOneMinusDestinationColor = 7,
    MTLBlendFactorDestinationAlpha = 8,
    MTLBlendFactorOneMinusDestinationAlpha = 9,
    MTLBlendFactorSourceAlphaSaturated = 10,
    MTLBlendFactorBlendColor = 11,
    MTLBlendFactorOneMinusBlendColor = 12,
    MTLBlendFactorBlendAlpha = 13,
    MTLBlendFactorOneMinusBlendAlpha = 14,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLBlendOperation {
    MTLBlendOperationAdd = 0,
    MTLBlendOperationSubtract = 1,
    MTLBlendOperationReverseSubtract = 2,
    MTLBlendOperationMin = 3,
    MTLBlendOperationMax = 4,
}

bitflags! {
    flags MTLColorWriteMask: NSUInteger {
        const MTLColorWriteMaskNone  = 0,
        const MTLColorWriteMaskRed   = 0x1 << 3,
        const MTLColorWriteMaskGreen = 0x1 << 2,
        const MTLColorWriteMaskBlue  = 0x1 << 1,
        const MTLColorWriteMaskAlpha = 0x1 << 0,
        const MTLColorWriteMaskAll   = 0xf
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLPrimitiveTopologyClass {
    MTLPrimitiveTopologyClassUnspecified = 0,
    MTLPrimitiveTopologyClassPoint = 1,
    MTLPrimitiveTopologyClassLine = 2,
    MTLPrimitiveTopologyClassTriangle = 3,
}

pub trait MTLRenderPipelineColorAttachmentDescriptor {
    unsafe fn pixelFormat(self) -> MTLPixelFormat;
    unsafe fn setPixelFormat(self, pixelFormat: MTLPixelFormat);

    unsafe fn blendingEnabled(self) -> BOOL;
    unsafe fn setBlendingEnabled(self, blendingEnabled: BOOL);

    unsafe fn sourceRGBBlendFactor(self) -> MTLBlendFactor;
    unsafe fn setSourceRGBBlendFactor(self, sourceRGBBlendFactor: MTLBlendFactor);

    unsafe fn destinationRGBBlendFactor(self) -> MTLBlendFactor;
    unsafe fn setDestinationRGBBlendFactor(self, destinationRGBBlendFactor: MTLBlendFactor);

    unsafe fn rgbBlendOperation(self) -> MTLBlendOperation;
    unsafe fn setRgbBlendOperation(self, rgbBlendOperation: MTLBlendOperation);

    unsafe fn sourceAlphaBlendFactor(self) -> MTLBlendFactor;
    unsafe fn setSourceAlphaBlendFactor(self, sourceAlphaBlendFactor: MTLBlendFactor);

    unsafe fn destinationAlphaBlendFactor(self) -> MTLBlendFactor;
    unsafe fn setDestinationAlphaBlendFactor(self, destinationAlphaBlendFactor: MTLBlendFactor);

    unsafe fn alphaBlendOperation(self) -> MTLBlendOperation;
    unsafe fn setAlphaBlendOperation(self, alphaBlendOperation: MTLBlendOperation);

    unsafe fn writeMask(self) -> MTLColorWriteMask;
    unsafe fn setWriteMask(self, writeMask: MTLColorWriteMask);
}

