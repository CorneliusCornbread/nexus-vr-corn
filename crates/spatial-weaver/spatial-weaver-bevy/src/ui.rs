use bevy::{
	ecs::{
		component::Component,
		entity::Entity,
		event::EventReader,
		system::{Query, Res},
	},
	math::Vec3,
	time::Time,
	transform::components::Transform,
};
use bevy_rapier3d::{
	pipeline::{CollisionEvent, ContactForceEvent},
	plugin::RapierContext,
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

fn check_ui_intersections(
	pointer: Query<(Entity, &Interactor3D)>,
	rapier_context: Res<RapierContext>,
) {
	// TODO: change this later, we're going to support multiple pointers, I'm just lazy
	let entity = pointer
		.get_single()
		.expect("FIXME: multiple pointers detected in scene");

	/* Iterate through all the intersection pairs involving a specific collider. */
	for (collider1, collider2, intersecting) in
		rapier_context.intersection_pairs_with(entity.0)
	{
		if intersecting {
			println!(
				"The entities {:?} and {:?} have intersecting colliders!",
				collider1, collider2
			);
		}
	}
}
