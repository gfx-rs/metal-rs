use metal::*;
fn main() {
    let device = Device::system_default().expect("No device found");

    let counter_sample_buffer = create_counter_sample_buffer(&device);

    //Apple silicon uses at stage boundary
    let counter_sampling_point = MTLCounterSamplingPoint::AtStageBoundary;
    assert!(device.supports_counter_sampling(counter_sampling_point));

    let command_queue = device.new_command_queue();

    let data = [1u32; 64 * 64];

    let buffer = device.new_buffer_with_data(
        unsafe { std::mem::transmute(data.as_ptr()) },
        (data.len() * std::mem::size_of::<u32>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );

    let sum = {
        let data = [0u32];
        device.new_buffer_with_data(
            unsafe { std::mem::transmute(data.as_ptr()) },
            (data.len() * std::mem::size_of::<u32>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    };

    let command_buffer = command_queue.new_command_buffer();

    let compute_pass_descriptor = ComputePassDescriptor::new();
    let sample_buffer_attachments = compute_pass_descriptor.sample_buffer_attachments();
    let sample_buffer_attachment_descriptor = sample_buffer_attachments.object_at(0).unwrap();

    sample_buffer_attachment_descriptor.set_sample_buffer(&counter_sample_buffer);
    sample_buffer_attachment_descriptor.set_start_of_encoder_sample_index(0);
    sample_buffer_attachment_descriptor.set_end_of_encoder_sample_index(1);

    let encoder = command_buffer.compute_command_encoder_with_descriptor(&compute_pass_descriptor);
    let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/compute/shaders.metallib");

    let library = device.new_library_with_file(library_path).unwrap();
    let kernel = library.get_function("sum", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    let pipeline_state = device
        .new_compute_pipeline_state_with_function(
            pipeline_state_descriptor.compute_function().unwrap(),
        )
        .unwrap();

    encoder.set_compute_pipeline_state(&pipeline_state);
    encoder.set_buffer(0, Some(&buffer), 0);
    encoder.set_buffer(1, Some(&sum), 0);

    let width = 16;

    let thread_group_count = MTLSize {
        width,
        height: 1,
        depth: 1,
    };

    let thread_group_size = MTLSize {
        width: (data.len() as u64 + width) / width,
        height: 1,
        depth: 1,
    };

    encoder.dispatch_thread_groups(thread_group_count, thread_group_size);
    encoder.end_encoding();

    let blit_encoder = command_buffer.new_blit_command_encoder();
    let destination_buffer = device.new_buffer(
        (std::mem::size_of::<u64>() * 2) as u64,
        MTLResourceOptions::StorageModeShared,
    );
    let range = crate::NSRange::new(0_u64, 2_u64);
    blit_encoder.resolve_counters(&counter_sample_buffer, range, &destination_buffer, 0_u64);
    blit_encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    let timestamps =
        unsafe { std::slice::from_raw_parts(destination_buffer.contents() as *const u64, 2) };
    println!("Start timestamp: {}", timestamps[0]);
    println!("End timestamp:   {}", timestamps[1]);
    println!("Elapsed time:    {}", timestamps[1] - timestamps[0]);

    let ptr = sum.contents() as *mut u32;
    println!("Compute shader sum: {}", unsafe { *ptr });

    unsafe {
        assert_eq!(4096, *ptr);
    }
}

fn create_counter_sample_buffer(device: &Device) -> CounterSampleBuffer {
    let counter_sample_buffer_desc = metal::CounterSampleBufferDescriptor::new();
    counter_sample_buffer_desc.set_storage_mode(metal::MTLStorageMode::Shared);
    counter_sample_buffer_desc.set_sample_count(2_u64);
    counter_sample_buffer_desc.set_counter_set(&fetch_timestamp_counter_set(device));

    device
        .new_counter_sample_buffer_with_descriptor(&counter_sample_buffer_desc)
        .unwrap()
}

fn fetch_timestamp_counter_set(device: &Device) -> metal::CounterSet {
    let counter_sets = device.counter_sets();
    let mut timestamp_counter = None;
    for cs in counter_sets.iter() {
        if cs.name() == "timestamp" {
            timestamp_counter = Some(cs);
            break;
        }
    }
    timestamp_counter
        .expect("No timestamp counter found")
        .clone()
}
