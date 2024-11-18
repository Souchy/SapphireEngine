use bevy_ecs::bundle::Bundle;

use crate::components::{Children, Parent, Transform2D};


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
#[derive(Bundle)]
struct Node2DBundle {
	parent: Parent,
	children: Children,
	transform: Transform2D
}

// pub struct Node2D;
// impl Node for Node2D {
// 	fn update(&mut self, delta: f32) {

// 	}
// }
