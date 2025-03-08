//! Renders a textured quad to a window and adjusts the GPU buffer that contains the viewport's
//! height and width whenever the window is resized.

use std::mem;
use std::path::PathBuf;

use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use objc::{rc::autoreleasepool, runtime::YES};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::Window;

use metal::{
    Buffer, CommandQueue, Device, Library, MTLClearColor, MTLLoadAction, MTLOrigin, MTLPixelFormat,
    MTLPrimitiveType, MTLRegion, MTLResourceOptions, MTLSize, MTLStoreAction, MetalLayer,
    MetalLayerRef, RenderPassDescriptor, RenderPassDescriptorRef, RenderPipelineDescriptor,
    RenderPipelineState, Texture, TextureDescriptor, TextureRef,
};
use shader_bindings::{
    TextureIndex_TextureIndexBaseColor as TextureBaseColorIdx, TexturedVertex,
    VertexInputIndex_VertexInputIndexVertices as VerticesBufferIdx,
    VertexInputIndex_VertexInputIndexViewportSize as ViewportSizeBufferIdx,
};
use winit::event::{Event, WindowEvent};

mod shader_bindings;

const INITIAL_WINDOW_WIDTH: u32 = 800;
const INITIAL_WINDOW_HEIGHT: u32 = 600;
const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = init_window(&event_loop);

    let device = Device::system_default().expect("No device found");
    let library = device.new_library_with_file(shader_metallib()).unwrap();

    let vertex_data = vertices();
    let vertex_buffer = device.new_buffer_with_data(
        vertex_data.as_ptr().cast(),
        size_of_val(&vertex_data) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
    );

    let viewport_size_buffer = device.new_buffer(
        8,
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
    );
    let viewport_size = (window.inner_size().width, window.inner_size().height);
    update_viewport_size_buffer(&viewport_size_buffer, viewport_size);

    let texture_to_render = create_texture_to_display(&device);

    let layer = get_window_layer(&window, &device);
    let pipeline_state = prepare_pipeline_state(&device, &library);
    let command_queue = device.new_command_queue();

    event_loop
        .run(move |event, event_loop| {
            autoreleasepool(|| {
                event_loop.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::AboutToWait => window.request_redraw(),
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::Resized(size) => {
                            layer.set_drawable_size(CGSize::new(
                                size.width as f64,
                                size.height as f64,
                            ));
                            update_viewport_size_buffer(
                                &viewport_size_buffer,
                                (size.width, size.height),
                            );
                        }
                        WindowEvent::RedrawRequested => redraw(
                            &layer,
                            &pipeline_state,
                            &command_queue,
                            &vertex_buffer,
                            &viewport_size_buffer,
                            &texture_to_render,
                        ),
                        _ => {}
                    },
                    _ => {}
                }
            });
        })
        .unwrap();
}

// The `TexturedVertex` type is generated by `build.rs` by parsing `shader_types.h`
// using rust-bindgen
fn vertices() -> [TexturedVertex; 6] {
    [
        textured_vertex([-200., -200.], [0., 1.]),
        textured_vertex([200., -200.], [1., 1.]),
        textured_vertex([200., 200.], [1., 0.]),
        textured_vertex([-200., -200.], [0., 1.]),
        textured_vertex([200., 200.], [1., 0.]),
        textured_vertex([-200., 200.], [0., 0.]),
    ]
}

fn textured_vertex(position: [f32; 2], texture_coord: [f32; 2]) -> TexturedVertex {
    unsafe {
        // The metal shader is expecting two floats, but the rust-bindgen generated
        // type is a u64.
        //
        // So, we transmute the 2 floats into a u64 so that when the shader receives
        // these 64 bits they'll be interpreted as a `vector_float2`.
        TexturedVertex {
            position: std::mem::transmute(position),
            texture_coord: std::mem::transmute(texture_coord),
        }
    }
}

fn create_texture_to_display(device: &Device) -> Texture {
    let img = include_bytes!("../gfx-rs.png");
    let decoder = png::Decoder::new(img.as_ref());
    let mut reader = decoder.read_info().unwrap();

    let info = reader.info();
    let (width, height) = (info.width as u64, info.height as u64);

    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    for idx in 0..buf.len() / 4 {
        let idx = idx * 4;

        buf.swap(idx, idx + 2);
    }

    let texture = TextureDescriptor::new();
    texture.set_width(width);
    texture.set_height(height);
    texture.set_pixel_format(PIXEL_FORMAT);

    let texture = device.new_texture(&texture);

    texture.replace_region(
        MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size: MTLSize {
                width,
                height,
                depth: 1,
            },
        },
        0,
        buf.as_ptr() as _,
        width * 4,
    );

    texture
}

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let window_size = LogicalSize::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    winit::window::WindowBuilder::new()
        .with_inner_size(window_size)
        .with_title("Creating and Sampling Textures Example".to_string())
        .build(event_loop)
        .unwrap()
}

fn get_window_layer(window: &Window, device: &Device) -> MetalLayer {
    let layer = MetalLayer::new();

    layer.set_device(device);
    layer.set_pixel_format(PIXEL_FORMAT);
    // Presenting with transactions isn't necessary since we aren't synchonizing with other UIKit
    // draw calls.
    // https://developer.apple.com/documentation/quartzcore/cametallayer/1478157-presentswithtransaction
    layer.set_presents_with_transaction(false);

    layer.set_drawable_size(CGSize::new(
        window.inner_size().width as f64,
        window.inner_size().height as f64,
    ));

    unsafe {
        if let Ok(RawWindowHandle::AppKit(rw)) = window.window_handle().map(|wh| wh.as_raw()) {
            let view = rw.ns_view.as_ptr() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(mem::transmute(layer.as_ref()));
        }
    }

    layer
}

fn shader_metallib() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shaders.metallib")
}

fn prepare_pipeline_state(device: &Device, library: &Library) -> RenderPipelineState {
    let pipeline_state_descriptor = RenderPipelineDescriptor::new();

    let vertex_shader = library.get_function("quad_vertex", None).unwrap();
    let fragment_shader = library.get_function("sampling_shader", None).unwrap();

    pipeline_state_descriptor.set_vertex_function(Some(&vertex_shader));
    pipeline_state_descriptor.set_fragment_function(Some(&fragment_shader));

    pipeline_state_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap()
        .set_pixel_format(PIXEL_FORMAT);

    device
        .new_render_pipeline_state(&pipeline_state_descriptor)
        .unwrap()
}

fn update_viewport_size_buffer(viewport_size_buffer: &Buffer, size: (u32, u32)) {
    let contents = viewport_size_buffer.contents();
    let viewport_size: [u32; 2] = [size.0, size.1];
    let byte_count = size_of_val(&viewport_size);

    unsafe {
        std::ptr::copy(viewport_size.as_ptr(), contents.cast(), byte_count);
    }
    viewport_size_buffer.did_modify_range(metal::NSRange::new(0, byte_count as u64));
}

fn redraw(
    layer: &MetalLayerRef,
    pipeline_state: &RenderPipelineState,
    command_queue: &CommandQueue,
    vertex_buffer: &Buffer,
    viewport_size_buffer: &Buffer,
    texture_to_render: &TextureRef,
) {
    let drawable = match layer.next_drawable() {
        Some(drawable) => drawable,
        None => return,
    };

    let render_pass_descriptor = RenderPassDescriptor::new();
    prepare_render_pass_descriptor(render_pass_descriptor, drawable.texture());

    let command_buffer = command_queue.new_command_buffer();

    let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
    encoder.set_render_pipeline_state(&pipeline_state);

    encoder.set_vertex_buffer(VerticesBufferIdx as u64, Some(vertex_buffer), 0);
    encoder.set_vertex_buffer(ViewportSizeBufferIdx as u64, Some(viewport_size_buffer), 0);
    encoder.set_fragment_texture(TextureBaseColorIdx as u64, Some(texture_to_render));

    encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 6);
    encoder.end_encoding();

    command_buffer.present_drawable(&drawable);
    command_buffer.commit();
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.2, 0.5, 0.8, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}
