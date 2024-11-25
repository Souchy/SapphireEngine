use bevy_ecs::bundle::Bundle;

use crate::components::{Batch, Children, Parent, Transform2D, Transform3D};
pub mod batch;

// pub trait Node {
// 	fn update(&mut self, delta: f32) {

// 	}
// 	// fn render(delta: f32);
// }

#[derive(Bundle)]
struct NodeBundle {
	parent: Parent,
	children: Children
}

// 2d -----------------------------------------------------------------------------------------------------
#[derive(Bundle)]
struct Node2dBundle {
	parent: Parent,
	children: Children,
	transform: Transform2D
}
#[derive(Bundle)]
struct Node3dBundle {
	parent: Parent,
	children: Children,
	transform: Transform3D
}
// pub struct Node2D;
// impl Node for Node2D {
// 	fn update(&mut self, delta: f32) {
// 	}
// }
#[derive(Bundle)]
struct Node2dBatchBundle {
	parent: Parent,
	children: Children,
	transform: Transform2D,
	instances: Batch
}
// 3d -----------------------------------------------------------------------------------------------------
#[derive(Bundle)]
struct Node3dBatchBundle {
	parent: Parent,
	children: Children,
	transform: Transform3D,
	instances: Batch
}
