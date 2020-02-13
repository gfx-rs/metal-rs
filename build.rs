use std::process::Command;

fn build_example_shader(example: &str) {
    let example = example.to_string();
    println!("cargo:rerun-if-changed=examples/{}/shaders.metal", example);

    let shader = format!("examples/{}/shaders.metal", example);
    let air = format!("examples/{}/shaders.air", example);
    let lib = format!("examples/{}/shaders.metallib", example);

    let air_status = Command::new("xcrun")
        .args(&["-sdk", "macosx", "metal", "-c", &shader, "-o", &air])
        .status()
        .expect("failed to execute metal compiler");
    if !air_status.success() {
        panic!("Failed to compile .metal -> .air");
    }

    let metal_status = Command::new("xcrun")
        .args(&["-sdk", "macosx", "metallib", &air, "-o", &lib])
        .status()
        .expect("failed to execute metal combine compiler");
    if !metal_status.success() {
        panic!("Failed .air to .metallib");
    }
}

fn build_example_shaders() {
    build_example_shader("compute");
    build_example_shader("window");
}

fn main() {
    #[cfg(target_os = "macos")]
    build_example_shaders();
}