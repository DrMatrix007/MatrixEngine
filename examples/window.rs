use matrix_engine::engine::{Engine, runtime::SingleThreadedRuntime};
use winit::{event_loop::EventLoop, window::WindowAttributes};
use winit::event_loop::ControlFlow;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    
    let mut engine = Engine::new(SingleThreadedRuntime);
    
    
    event_loop.run_app(&mut engine).unwrap();
}
