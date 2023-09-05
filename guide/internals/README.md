Under the hood, `metal-rs` is powered by three main dependencies.

[`objc2`] and [`block2`] and [`foreign-types`].

These dependencies are used to communicate with the [Objective-C runtime] in order to allocate, de-allocate and call methods on the classes and objects that power Metal applications.

## Memory Management

`metal-rs` follows Apple's [memory management policy].

When calling a method such as `CaptureManager::shared()`, the implementation looks like this:

```rust
impl CaptureManager {
    pub fn shared<'a>() -> &'a CaptureManagerRef {
        unsafe {
            let class = class!(MTLCaptureManager);
            msg_send![class, sharedCaptureManager]
        }
    }
}
```

Note that a borrowed reference is returned. As such, when the returned reference is dropped, memory will not be deallocated.

Contrast this with the `StencilDescriptor::new()` method.

```rust
impl StencilDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLStencilDescriptor);
            msg_send![class, new]
        }
    }
}
```

In this case we are calling the `new` method on a class, which returns an owned object.

The macro

```rust
foreign_obj_type! {
    type CType = MTLStencilDescriptor;
    pub struct StencilDescriptor;
    pub struct StencilDescriptorRef;
}
```

ensures that when the owned `StencilDescriptor` is dropped it will call `release` on the backing Objective-C object, leading to the memory being deallocated.

[`objc2`]: https://crates.io/crates/objc2
[`block2`]: https://crates.io/crates/block2
[`foreign-types`]: https://crates.io/crates/foreign-types
[Objective-C runtime]: https://developer.apple.com/documentation/objectivec/objective-c_runtime
[memory management policy]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html#//apple_ref/doc/uid/20000994-BAJHFBGH
