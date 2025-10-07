use wgpu::RenderPass;

pub trait Passable {
    fn apply<'a>(&self, pass: &mut RenderPass<'a>);
}