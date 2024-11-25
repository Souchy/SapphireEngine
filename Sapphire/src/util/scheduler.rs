use bevy_ecs::prelude::*;
use std::time::{Duration, Instant};
use tokio::task;
use tokio::time;

// A resource to hold system scheduling information
#[derive(Default, Resource)]
struct Scheduler {
    fps_systems: Vec<(u32, Box<dyn FnMut(&mut World) + Send + Sync>)>,
}

// pub trait Looper {
//     fn start(&mut self, w: &mut World) -> tokio::task::JoinHandle<()>;
// }

// impl Looper for Schedule {
//     fn start(&mut self, w: &mut World) -> tokio::task::JoinHandle<()> {
//         tokio::spawn(async move {
//             loop {
//                 unsafe {
//                     // self.run(w);
//                     // w.run_schedule(label);
//                     self.run(w);
//                 }
//             }
//         })
//     }
// }

// impl Scheduler {
//     fn new() -> Self {
//         Self {
//             fps_systems: Vec::new(),
//         }
//     }

//     fn add_system<F>(&mut self, fps: u32, mut system: F)
//     where
//         F: FnMut(&mut World) + Send + Sync + 'static,
//     {
//         self.fps_systems
//             .push((fps, Box::new(move |world| system(world))));
//     }

//     async fn run(mut self, mut world: World) {
//         for (fps, mut system) in self.fps_systems.drain(..) {
//             let interval = Duration::from_secs_f32(1.0 / fps as f32);
//             let mut world_clone = world.clone(); // To use world safely across tasks
//             task::spawn(async move {
//                 let mut timer = time::interval(interval);
//                 timer.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

//                 loop {
//                     timer.tick().await;
//                     system(&mut world_clone);
//                 }
//             });
//         }

//         // Keep the app running to allow tasks to execute
//         let _ = task::spawn_blocking(move || loop {
//             std::thread::park(); // Keeps the runtime alive
//         })
//         .await;
//     }
// }
