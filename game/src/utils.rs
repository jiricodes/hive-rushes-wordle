use bevy::prelude::*;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	// println!("despawn_screen");
	for entity in to_despawn.iter() {
		commands.entity(entity).despawn_recursive();
	}
}
