use std::collections::HashMap;

use crate::components::{resources::{model::Model, sprite::Sprite}, Transform2D, Transform3D};


struct Batch2d {
	pub sprite: Sprite,
	pub transforms: Vec<Transform2D>,
	pub properties: BatchInstanceProperties
}

struct Batch3d {
	pub model: Model,
	pub transforms: Vec<Transform3D>,
	// maybe better as individuel entities in the ECS ? 
	// just idk how to group them to batch draw if i do that
	// maybe a batch Node with an Instances component that links to other entities, just like Children
	pub properties: BatchInstanceProperties
}

struct BatchInstanceProperties {
	pub dic: HashMap<String, String>
}
