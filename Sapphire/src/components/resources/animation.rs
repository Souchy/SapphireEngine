use bevy_ecs::component::Component;

#[derive(Component, Debug, Default, Clone)]
pub struct Animation {
	pub name: String,
	pub length: f32,
	pub looping: bool,
	// toutes les transforms de bones
}


#[derive(Component, Debug, Default, Clone)]
pub struct AnimationPlayer {
	pub currentAnimation: String,
	pub currentTime: String
}
