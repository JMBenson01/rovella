use crate::platform::Window;

use std::borrow::{Borrow, Cow};
use std::path::Iter;
use std::convert::AsRef;
use std::iter;
use std::ops::Deref;

use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages, Instance, TextureViewDimension, VertexState, VertexStepMode};


pub struct Renderer {
    context: RenderContext,
}

impl Renderer {
    #[inline]
    pub fn new(win: &Window) -> Option<Renderer> {
        let context_op = futures::executor::block_on(RenderContext::new(win));
        if context_op.is_none() {
            return None;
        }

        let context = context_op.unwrap();

        Some(Renderer { context })
    }


    #[inline]
    pub fn render(&mut self) {
        self.context.render();
    }

}

pub struct RenderContext {
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: wgpu::RenderPipeline,
}

impl RenderContext {
    pub(crate) async fn new(win: &Window) -> Option<RenderContext> {
        let inst = Instance::new(wgpu::Backends::VULKAN);

        let surface = unsafe { inst.create_surface(win) };

        let adap_op = inst
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await;

        if adap_op.is_none() {
            log_error!("Failed to create adapter for renderer");
            return None;
        }

        let adapter = adap_op.unwrap();

        let res = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await;

        if res.is_err() {
            log_error!("Failed to creat renderer Device and Queue");
            return None;
        }

        let (device, queue) = res.unwrap();

        surface.configure(&device, &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: win.get_width() as u32,
            height: win.get_height() as u32,
            present_mode: wgpu::PresentMode::Fifo
        });

        let shader_module = device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/vertex_shader.wgsl"))),
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[]
            })),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::TextureFormat::Bgra8UnormSrgb.into())]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });


        return Some(RenderContext {
            surface,
            adapter,
            device,
            queue,
            pipeline
        });
    }

    fn render(&mut self) {
        let mut command_encoder = self.
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let tex_res = self.surface.get_current_texture();

        if tex_res.is_err() {
            if tex_res.as_ref().err().is_some() {
                log_error!("Failed to retrieve texture from surface: {} ", tex_res.as_ref().err().unwrap());
            } else {
                log_error!("Failed to retrieve texture from surface");
            }
            return;
        } else {
            log_info!("Not error");
        }

        let tex = tex_res.unwrap();
        let view = tex.texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(wgpu::TextureFormat::Bgra8UnormSrgb),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: Default::default(),
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None
        });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0
                        }),
                        store: true
                    }
                })],
                depth_stencil_attachment: None
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(iter::once(command_encoder.finish()));
    }

    #[inline]
    pub fn create_shader_module_from_file(&self, file_name: &'static str) -> wgpu::ShaderModule {
        return self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(file_name)),
            });
    }
}
