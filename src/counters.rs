/// See <https://developer.apple.com/documentation/metal/mtlcountersamplebuffer>
pub enum MTLCounterSampleBuffer {}

foreign_obj_type! {
    type CType = MTLCounterSampleBuffer;
    pub struct CounterSampleBuffer;
}

impl CounterSampleBufferRef {
    pub fn resolve_counter_range(&self, range: crate::NSRange) {
        unsafe { msg_send![self, resolveCountersInRange: range] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlcounter>
pub enum MTLCounter {}

foreign_obj_type! {
    type CType = MTLCounter;
    pub struct Counter;
}

/// See <https://developer.apple.com/documentation/metal/mtlcounterset>
pub enum MTLCounterSet {}

foreign_obj_type! {
    type CType = MTLCounterSet;
    pub struct CounterSet;
}

impl CounterSetRef {}

/// See <https://developer.apple.com/documentation/metal/mtlcommoncounterset>
pub enum MTLCommonCounterSet {}

/// See <https://developer.apple.com/documentation/metal/mtlcommoncounter>
pub enum MTLCommonCounter {}

foreign_obj_type! {
    type CType = MTLCommonCounter;
    pub struct CommonCounter;
}
