use metal::*;
fn main() {
    timestamp_exp();
    let device = Device::system_default().expect("No device found");

    let counter_sample_buffer_desc = metal::CounterSampleBufferDescriptor::new();
    counter_sample_buffer_desc.set_storage_mode(metal::MTLStorageMode::Shared);
    counter_sample_buffer_desc.set_sample_count(2_u64);
    counter_sample_buffer_desc.set_counter_set(&fetch_timestamp_counter_set(&device));

    let csb = device
        .new_counter_sample_buffer_with_descriptor(&counter_sample_buffer_desc)
        .unwrap();

    //Apple silicon uses at stage boundary
    let counter_sampling_point = MTLCounterSamplingPoint::AtStageBoundary;
    let supported = device.supports_counter_sampling(counter_sampling_point);
    println!("Supports stage boundary sampling: {}", supported);

    let command_queue = device.new_command_queue();

    let data = [
        1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30,
    ];

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

    command_buffer.set_label("label");
    let block = block::ConcreteBlock::new(move |buffer: &metal::CommandBufferRef| {
        println!("{}", buffer.label());
    })
    .copy();

    command_buffer.add_completed_handler(&block);

    let compute_pass_descriptor = ComputePassDescriptor::new();
    let sample_buffer_attachments = compute_pass_descriptor.sample_buffer_attachments();
    let sample_buffer_attachment_descriptor = sample_buffer_attachments.object_at(0).unwrap();

    sample_buffer_attachment_descriptor.set_sample_buffer(&csb);
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
        MTLResourceOptions::StorageModePrivate,
    );
    let range = crate::NSRange::new(0_u64, 2_u64);
    blit_encoder.resolve_counters(&csb, range, &destination_buffer, 0_u64);
    blit_encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    let samples = destination_buffer.contents() as *mut u32;
    println!("samples: {:?}", unsafe { *samples });

    let ptr = sum.contents() as *mut u32;
    println!("sum: {}", unsafe { *ptr });
    unsafe {
        assert_eq!(465, *ptr);
    }
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

fn timestamp_exp() {
    let device = Device::system_default().expect("No device found");
    let (mut cpu_timestamp0, mut gpu_timestamp0) = (0_u64, 0_u64);
    device.sample_timestamps(&mut cpu_timestamp0, &mut gpu_timestamp0);
    println!(
        "cpu_timestamp0: {}, gpu_timestamp0: {}",
        cpu_timestamp0, gpu_timestamp0
    );

    let (mut cpu_timestamp1, mut gpu_timestamp1) = (0_u64, 0_u64);
    device.sample_timestamps(&mut cpu_timestamp1, &mut gpu_timestamp1);
    println!(
        "cpu_timestamp1: {}, gpu_timestamp1: {}",
        cpu_timestamp1, gpu_timestamp1
    );
}
