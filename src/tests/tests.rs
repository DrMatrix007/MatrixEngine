use crate::matrix_engine::{
    application::Application,
    layer::{Layer, LayerArgs},
};

struct FpsLayer {}

impl FpsLayer {
    fn new() -> Self {
        FpsLayer {}
    }
}
impl Layer for FpsLayer {
    fn on_start(&mut self, mut _args: &LayerArgs) {}

    fn on_update(&mut self, _args: &LayerArgs) {

        println!("fps: {}", 1.0/_args.delta_time.as_secs_f64());        

        if _args.to_owned().time.as_secs() >= 1 {
            println!("stopped!");
            _args.stop_application();
        }
    }

    fn on_clean_up(&mut self) {}
}

#[test]
fn test_preformace() {
    let mut app = Application::new();

    app.push_layer(FpsLayer::new());

    app.run();
}
#[test]
fn test_box() {}
