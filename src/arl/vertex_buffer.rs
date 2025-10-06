pub trait VertexBuffer {
    fn desc() -> (wgpu::VertexStepMode, Vec<wgpu::VertexFormat>);
}

pub trait VertexBufferGroup {
    type ATTRS;

    fn attrs() -> Self::ATTRS;

    fn desc<'a>(attrs: &'a Self::ATTRS) -> Vec<wgpu::VertexBufferLayout<'a>>;
}

impl<T: VertexBuffer> VertexBufferGroup for T {
    type ATTRS = (wgpu::VertexStepMode, Vec<wgpu::VertexAttribute>, u64);

    fn attrs() -> Self::ATTRS {
        let d = T::desc();
        let mut addr_offest = 0;
        let mut shader_location = 0;
        (
            d.0,
            d.1.iter()
                .map(move |format| {
                    let attr = wgpu::VertexAttribute {
                        format: *format,
                        offset: addr_offest,
                        shader_location,
                    };
                    shader_location += 1;
                    addr_offest += format.size();

                    attr
                })
                .collect(),
            addr_offest,
        )
    }

    fn desc<'a>(attrs: &'a Self::ATTRS) -> Vec<wgpu::VertexBufferLayout<'a>> {
        vec![wgpu::VertexBufferLayout {
            array_stride: attrs.2,
            attributes: &attrs.1,
            step_mode: attrs.0,
        }]
    }
}
