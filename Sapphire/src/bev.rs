use std::time::Duration;

use bevy_ecs::{
    entity::Entity,
    schedule::Schedule,
    system::Query,
    world::World,
};

use crate::components::{resources::mesh::Mesh, Position, Velocity};

pub static mut WORLDO: Option<World> = None;

pub async fn bev() {
    println!("Hello, world!");
    // App::new().add_systems(Update, hello_world).run();
    let mut world = World::default();

    let entityId = world
        .spawn((
            Position { x: 0.0, y: 0.0 },
            Velocity { x: 1.0, y: 0.0 },
            Mesh {},
        ))
        .id();

    // let entity_ref = world.entity(entity);
    // let position = entity_ref.get::<Position>().unwrap();
    // let velocity = entity_ref.get::<Velocity>().unwrap();

    // let mut commands: Commands;
    // commands.entity(entity).despawn();

    unsafe {
        WORLDO = Some(world);
    }
    let mut update_schedule = Schedule::default();
    update_schedule.add_systems(movement);
    update_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::MultiThreaded);
    let t1 = start_schedule(update_schedule);

    let mut render_schedule: Schedule = Schedule::default();
    render_schedule.add_systems(render_mesh);
    render_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::Simple);
    let t2 = start_schedule(render_schedule);

    let (_res1, _res2) = tokio::join!(t1, t2);
    // Ok(())
}

fn start_schedule(mut s: Schedule) -> tokio::task::JoinHandle<()> {
    return tokio::spawn(async move {
        loop {
            // println!("t1");
            unsafe {
                if let Some(w) = &mut WORLDO {
                    s.run(w);
                }
            }
        }
    });
}

fn hello_world() {
    println!("hello world!");
}
fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, position) in &query {
        println!(
            "Entity {:?} is at position: x {}, y {}",
            entity, position.x, position.y
        );
    }
}
// This system moves each entity with a Position and Velocity component
fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
        println!("movement {:?}", position);
    }
}

fn render_mesh(query: Query<(Entity, &Mesh)>) {
    println!("render_mesh");
}

fn setupSystems() {
    // let weekly = every(1).week().on(Weekday::Mon).at(12, 00, 00)
    //     .in_timezone(&Utc).perform(|| async { println!("Every week job") });
    // spawn(weekly);
    // let schedule = every(16).milliseconds().perform(|| async {
    //     println!("")
    // });
    // spawn(schedule);
    tokio::time::interval(Duration::from_millis(16));
}
