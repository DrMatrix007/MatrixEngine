use matrix_engine::engine::Engine;



fn main() {
    let engine = Engine<CustomEvents>::with_scene(Scene::new());

    engine.run().unwrap();

}