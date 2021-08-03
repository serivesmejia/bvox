use wgpu::Device;

pub struct Shader {
    pub module: wgpu::ShaderModule
}

impl Shader {
    pub fn new(device: &Device, shader: &str, label: &str) -> Self {
        let module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(label),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(shader.into()),
        });

        Self { module }
    }
}

#[macro_export]
macro_rules! shader {
    ($device:ident,$file_path:expr $(,)?,$label:expr $(,)?) => ({
        Shader::new(&$device, include_str!($file_path), $label)
    })
}