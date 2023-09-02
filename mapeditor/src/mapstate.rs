use graphics::*;

pub struct State<Controls>
where
    Controls: camera::controls::Controls,
{
    /// World Camera Controls and time. Deturmines how the world is looked at.
    pub system: System<Controls>,
    /// Data stores for render types
    pub sprites: Vec<Image>,
    pub texts: Vec<Text>,
    pub map: Map,

    /// Atlas Groups for Textures in GPU
    pub map_atlas: AtlasGroup,
    pub text_atlas: TextAtlas,
    /// Rendering Buffers and other shared data.
    pub text_renderer: TextRenderer,
    pub sprite_renderer: ImageRenderer,
    pub map_renderer: MapRenderer,
}

impl<Controls> Pass for State<Controls>
where
    Controls: camera::controls::Controls,
{
    fn render(&mut self, renderer: &GpuRenderer, encoder: &mut wgpu::CommandEncoder) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: renderer.frame_buffer().as_ref().expect("no frame view?"),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.25,
                        b: 0.5,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: renderer.depth_buffer(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0),
                    store: true,
                }),
            }),
        });

        // Lets set the System's Shader information here, mostly Camera, Size and Time
        pass.set_bind_group(0, self.system.bind_group(), &[]);
        // Lets set the Reusable Vertices and Indicies here.
        // This is used for each Renderer, Should be more performant since it is shared.
        pass.set_vertex_buffer(0, renderer.buffer_object.vertices());
        pass.set_index_buffer(renderer.buffer_object.indices(), wgpu::IndexFormat::Uint16);

        pass.render_lower_maps(renderer, &self.map_renderer, &self.map_atlas);
        pass.render_upper_maps(renderer, &self.map_renderer, &self.map_atlas);
        pass.render_text(renderer, &self.text_renderer, &self.text_atlas);
        pass.render_image(renderer, &self.sprite_renderer, &self.map_atlas);
    }
}
