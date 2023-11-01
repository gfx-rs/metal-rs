use metal::{CompileOptions, Library, Device};

fn main(){
    let source = include_str!("ggml-metal.metal");

    let device = Device::system_default().expect("No metal device found");
    let compile_options = CompileOptions::new();
    // TODO preprocessor macro

    let _library = Library::new(&device, source, &compile_options);

}
