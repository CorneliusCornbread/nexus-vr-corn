use std::process::Command;

use bevy::{
	ecs::{
		component::Component,
		system::{Query, Res},
	},
	math::Vec3,
	time::Time,
	transform::components::Transform,
};

use crate::Interactor3D;

#[derive(Component)]
pub struct BevyButton3D {
	min_pressure_percent: f32,
	pressure_release_percent: f32,
	current_pressure: f32,
	is_pressed: bool,
	collider_bounds: Vec3,
}

pub fn check_3D_interactions(
	interactors: Query<(&Transform, &Interactor3D)>,
	mut elements: Query<(&Transform, &mut BevyButton3D)>,
	time: Res<Time>,
) {
	for (transform, mut button) in elements.iter() {}
}
