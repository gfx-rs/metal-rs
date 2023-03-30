use metal::*;
use std::path::PathBuf;

const NUM_SAMPLES: u64 = 2;
const NUM_ELEMENTS: u64 = 64 * 64;

fn main() {
    let device = Device::system_default().expect("No device found");

    let counter_sample_buffer = create_counter_sample_buffer(&device);

    let counter_sampling_point = MTLCounterSamplingPoint::AtStageBoundary;
    assert!(device.supports_counter_sampling(counter_sampling_point));

    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    let compute_pass_descriptor = ComputePassDescriptor::new();
    handle_compute_pass_sample_buffer_attachment(&compute_pass_descriptor, &counter_sample_buffer);
    let encoder = command_buffer.compute_command_encoder_with_descriptor(&compute_pass_descriptor);

    let pipeline_state = create_pipeline_state(&device);
    encoder.set_compute_pipeline_state(&pipeline_state);

    let (buffer, sum) = create_input_and_output_buffers(&device);
    encoder.set_buffer(0, Some(&buffer), 0);
    encoder.set_buffer(1, Some(&sum), 0);

    let width = 16;

    let thread_group_count = MTLSize {
        width,
        height: 1,
        depth: 1,
    };

    let thread_group_size = MTLSize {
        width: (NUM_ELEMENTS + width) / width,
        height: 1,
        depth: 1,
    };

    encoder.dispatch_thread_groups(thread_group_count, thread_group_size);
    encoder.end_encoding();

    let resolved_sample_buffer =
        resolve_samples_into_buffer(&command_buffer, &counter_sample_buffer, &device);

    command_buffer.commit();
    command_buffer.wait_until_completed();

    print_timestamps(&resolved_sample_buffer);

    let ptr = sum.contents() as *mut u32;
    println!("Compute shader sum: {}", unsafe { *ptr });

    unsafe {
        assert_eq!(NUM_ELEMENTS as u32, *ptr);
    }
}

fn create_pipeline_state(device: &Device) -> ComputePipelineState {
    let library_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/compute/shaders.metallib");
    let library = device.new_library_with_file(library_path).unwrap();
    let kernel = library.get_function("sum", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    device
        .new_compute_pipeline_state_with_function(
            pipeline_state_descriptor.compute_function().unwrap(),
        )
        .unwrap()
}

fn handle_compute_pass_sample_buffer_attachment(
    compute_pass_descriptor: &ComputePassDescriptorRef,
    counter_sample_buffer: &CounterSampleBufferRef,
) {
    let sample_buffer_attachment_descriptor = compute_pass_descriptor
        .sample_buffer_attachments()
        .object_at(0)
        .unwrap();

    sample_buffer_attachment_descriptor.set_sample_buffer(&counter_sample_buffer);
    sample_buffer_attachment_descriptor.set_start_of_encoder_sample_index(0);
    sample_buffer_attachment_descriptor.set_end_of_encoder_sample_index(1);
}

fn resolve_samples_into_buffer(
    command_buffer: &CommandBufferRef,
    counter_sample_buffer: &CounterSampleBufferRef,
    device: &Device,
) -> Buffer {
    let blit_encoder = command_buffer.new_blit_command_encoder();
    let timestamps_buffer = device.new_buffer(
        (std::mem::size_of::<u64>() * NUM_SAMPLES as usize) as u64,
        MTLResourceOptions::StorageModeShared,
    );
    blit_encoder.resolve_counters(
        &counter_sample_buffer,
        crate::NSRange::new(0_u64, NUM_SAMPLES),
        &timestamps_buffer,
        0_u64,
    );
    blit_encoder.end_encoding();
    timestamps_buffer
}

fn print_timestamps(resolved_sample_buffer: &BufferRef) {
    let samples = unsafe {
        std::slice::from_raw_parts(
            resolved_sample_buffer.contents() as *const u64,
            NUM_SAMPLES as usize,
        )
    };
    let gpu_start = samples[0];
    let gpu_end = samples[1];
    println!("GPU start: {}", gpu_start);
    println!("GPU end: {}", gpu_end);

    //let micros = absolute_time_in_microseconds(cpu_start, cpu_end, gpu_start, gpu_end);
    let micros = 0;
    println!("CPU time: {} microseconds", micros);
}

fn create_counter_sample_buffer(device: &Device) -> CounterSampleBuffer {
    let counter_sample_buffer_desc = metal::CounterSampleBufferDescriptor::new();
    counter_sample_buffer_desc.set_storage_mode(metal::MTLStorageMode::Shared);
    counter_sample_buffer_desc.set_sample_count(NUM_SAMPLES);
    counter_sample_buffer_desc.set_counter_set(&fetch_timestamp_counter_set(device));

    device
        .new_counter_sample_buffer_with_descriptor(&counter_sample_buffer_desc)
        .unwrap()
}

fn fetch_timestamp_counter_set(device: &Device) -> metal::CounterSet {
    let counter_sets = device.counter_sets();
    let timestamp_counter = counter_sets.iter().find(|cs| cs.name() == "timestamp");
    timestamp_counter
        .expect("No timestamp counter found")
        .clone()
}

fn create_input_and_output_buffers(device: &Device) -> (metal::Buffer, metal::Buffer) {
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
    (buffer, sum)
}

/// <https://developer.apple.com/documentation/metal/gpu_counters_and_counter_sample_buffers/converting_gpu_timestamps_into_cpu_time>
fn absolute_time_in_microseconds(
    cpu_start: u64,
    cpu_end: u64,
    gpu_start: u64,
    gpu_end: u64,
) -> u64 {
    // Convert the GPU time to a value within the range [0.0, 1.0].
    let normalized_gpu_time = (gpu_end - gpu_start) / (gpu_end - gpu_start);

    // Convert GPU time to CPU time.
    let mut nanoseconds = normalized_gpu_time * (cpu_end - cpu_start);
    nanoseconds += cpu_start;

    let microseconds = nanoseconds / 1000;
    microseconds
}
