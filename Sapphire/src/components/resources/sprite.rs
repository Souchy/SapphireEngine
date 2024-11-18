use bevy_ecs::component::Component;
use image::DynamicImage;

#[derive(Component, Debug, Default, Clone)]
pub struct Sprite {
	img: DynamicImage
}
