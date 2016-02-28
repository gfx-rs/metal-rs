use cocoa::base::id;

pub trait MTLCommandQueue {
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    unsafe fn device(self) -> id;

    unsafe fn commandBuffer(self) -> id;
    unsafe fn commandBufferWithUnretainedReferences(self) -> id;

    unsafe fn insertDebugCaptureBoundary(self);
}

impl MTLCommandQueue for id {
    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }

    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn commandBuffer(self) -> id {
        msg_send![self, commandBuffer]
    }

    unsafe fn commandBufferWithUnretainedReferences(self) -> id {
        msg_send![self, commandBufferWithRetainedReferences]
    }

    unsafe fn insertDebugCaptureBoundary(self) {
        msg_send![self, insertDebugCaptureBoundary]
    }
}
