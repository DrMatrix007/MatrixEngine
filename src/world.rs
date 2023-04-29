// use std::sync::Arc;

// use crate::{
//     components::{
//         resources::ResourceRegistry,
//         storage::{Storage, StorageReadGuard, StorageWriteGuard},
//     },
//     dispatchers::{
//         dispatchers::DispatcherArgs,
//         system_registry::{BoxedExclusiveSystem, BoxedSystem, SystemGroup, SystemRegistryRefMut},
//         systems::{AsyncSystem, ExclusiveSystem, SystemArgs},
//     },
//     scene::Scene,
//     schedulers::schedulers::Scheduler,
// };

// pub(crate) struct WorldRefMut<'a> {
//     pub args: DispatcherArgs<'a>,
//     pub startups: &'a mut SystemGroup,
//     pub systems: &'a mut SystemGroup,
// }

// #[derive(Default)]
// pub struct World {
//     scene: Scene,
//     resources: Storage<ResourceRegistry>,
// }

// impl World {
//     pub fn new(scene: Scene, resources: ResourceRegistry) -> Self {
//         Self {
//             scene,
//             resources: Storage::new(resources),
//         }
//     }

   
//     pub fn scene(&self) -> &Scene {
//         &self.scene
//     }
//     pub fn scene_mut(&mut self) -> &mut Scene {
//         &mut self.scene
//     }
//     pub(crate) fn unpack(&mut self) -> WorldRefMut<'_> {
//         let World { scene, resources } = self;
//         let (sys, reg) = scene.unpack();
//         let SystemRegistryRefMut {
//             runtime_systems,
//             startup_systems,
//         } = sys.unpack();
//         WorldRefMut {
//             args: DispatcherArgs::new(reg, resources),
//             startups: startup_systems,
//             systems: runtime_systems,
//         }
//     }

//     pub fn resource_registry_mut(&mut self) -> Option<StorageWriteGuard<ResourceRegistry>> {
//         self.resources.write()
//     }
//     pub fn resource_registry(&self) -> Option<StorageReadGuard<ResourceRegistry>> {
//         self.resources.read()
//     }

//     pub(crate) fn run_startups(&mut self, scheduler: &mut dyn Scheduler) {
//         let startups = &mut self.scene.system_registry_mut().startup_systems;
//         scheduler.run(
//             startups,
//             &mut DispatcherArgs::new(&mut self.scene.components, &mut self.resources),
//             Arc::new(SystemArgs::new()),
//         )
//     }

//     pub(crate) fn run(&mut self) {}
// }
