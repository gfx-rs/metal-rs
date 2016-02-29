use cocoa::base::{id, BOOL};
use cocoa::foundation::{NSUInteger, NSTimeInterval};

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLCommandBufferStatus {
    MTLCommandBufferStatusNotEnqueued = 0,
    MTLCommandBufferStatusEnqueued = 1,
    MTLCommandBufferStatusCommitted = 2,
    MTLCommandBufferStatusScheduled = 3,
    MTLCommandBufferStatusCompleted = 4,
    MTLCommandBufferStatusError = 5,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLCommandBufferError {
    MTLCommandBufferErrorNone = 0,
    MTLCommandBufferErrorInternal = 1,
    MTLCommandBufferErrorTimeout = 2,
    MTLCommandBufferErrorPageFault = 3,
    MTLCommandBufferErrorBlacklisted = 4,
    MTLCommandBufferErrorNotPermitted = 7,
    MTLCommandBufferErrorOutOfMemory = 8,
    MTLCommandBufferErrorInvalidResource = 9,
}

type MTLCommandBufferHandler = extern fn(commandbuffer: id);

pub trait MTLCommandBuffer {
    unsafe fn device(self) -> id;
    unsafe fn commandQueue(self) -> id;
    unsafe fn retainedReferences(self) -> BOOL;
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    unsafe fn enqueue(self);
    unsafe fn commit(self);

    unsafe fn addScheduledHandler(self, block: MTLCommandBufferHandler);
    unsafe fn presentDrawable(self, drawable: id);
    unsafe fn presentDrawable_atTime(self, drawable: id, presentationTime: NSTimeInterval);
    unsafe fn waitUntilScheduled(self);
    
    unsafe fn addCompletedHandler(self, block: MTLCommandBufferHandler);
    unsafe fn waitUntilCompleted(self);

    unsafe fn status(self) -> MTLCommandBufferStatus;
    unsafe fn error(self) -> id;

    unsafe fn blitCommandEncoder(self) -> id;
    unsafe fn renderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id;
    unsafe fn computeCommandEncoder(self) -> id;
    unsafe fn parallelRenderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id;
}

impl MTLCommandBuffer for id {
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn commandQueue(self) -> id {
        msg_send![self, device]
    }

    unsafe fn retainedReferences(self) -> BOOL {
        msg_send![self, retainedReferences]
    }
    
    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }

    unsafe fn enqueue(self) {
        msg_send![self, enqueue]
    }

    unsafe fn commit(self) {
        msg_send![self, commit]
    }

    unsafe fn addScheduledHandler(self, block: MTLCommandBufferHandler) {
        msg_send![self, addScheduledHandler:block]
    }

    unsafe fn presentDrawable(self, drawable: id) {
        msg_send![self, presentDrawable:drawable]
    }

    unsafe fn presentDrawable_atTime(self, drawable: id, presentationTime: NSTimeInterval) {
        msg_send![self, presentDrawable:drawable
                        atTime:presentationTime]
    }

    unsafe fn waitUntilScheduled(self) {
        msg_send![self, waitUntilScheduled]
    }
    
    unsafe fn addCompletedHandler(self, block: MTLCommandBufferHandler) {
        msg_send![self, addCompletedHandler:block]
    }

    unsafe fn waitUntilCompleted(self) {
        msg_send![self, waitUntilCompleted]
    }

    unsafe fn status(self) -> MTLCommandBufferStatus {
        msg_send![self, status]
    }

    unsafe fn error(self) -> id {
        msg_send![self, error]
    }

    unsafe fn blitCommandEncoder(self) -> id {
        msg_send![self, blitCommandEncoder]
    }

    unsafe fn renderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id {
        msg_send![self, renderCommandEncoderWithDescriptor:renderPassDescriptor]
    }

    unsafe fn computeCommandEncoder(self) -> id {
        msg_send![self, computeCommandEncoder]
    }

    unsafe fn parallelRenderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id {
        msg_send![self, parallelRenderCommandEncoderWithDescriptor:renderPassDescriptor]
    }
}
