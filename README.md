# MATRIX ENGINE

Hi, this is a small game engine built in rust.
It contains ECS, and basic renderer. enjoy.

## BASIC USAGE

this is opens a basic window. implement your own Plugin for adding systems for the ECS.

```
fn main() {
    let mut engine = <Engine>::new(EngineArgs::new(SingleThreaded, SingleThreaded));

    engine.add_scene_plugin(WindowPlugin::new("hello example!"));

    engine.add_scene_plugin(RendererPlugin);
}
```
