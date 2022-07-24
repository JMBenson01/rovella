/*use wgpu::Instance;
use crate::platform::Window;

pub struct Renderer {
    surface: wgpu::Surface
}

impl Renderer {
    pub fn new(win: &Window) -> Renderer {
        let inst = Instance::new(wgpu::Backends::VULKAN);
        return Renderer {
            surface: unsafe { inst.create_surface(win) }
        }
    }
}
*/