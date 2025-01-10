use crate::models::components::{collider::Collider, position::Position};
use crate::models::entity::Entity;

pub fn check_collision(entity_a: &Entity, entity_b: &Entity) -> bool {
	let position_a = entity_a.get_component::<Position>();
	let collider_a = entity_a.get_component::<Collider>();

	let position_b = entity_b.get_component::<Position>();
	let collider_b = entity_b.get_component::<Collider>();

	if let (Some(position_a), Some(collider_a), Some(position_b), Some(collider_b)) =
		(position_a, collider_a, position_b, collider_b)
	{
		if position_a.map_id != position_b.map_id {
			return false;
		}

		let a_min = position_a.coords.add(collider_a.offset);
		let a_max = a_min.add(collider_a.size);

		let b_min = position_b.coords.add(collider_b.offset);
		let b_max = b_min.add(collider_b.size);

		a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
	} else {
		false
	}
}
