use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct MeleeEnemy {
    speed: f64,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for MeleeEnemy {
    fn init(base: Base<Self::Base>) -> Self {
        Self { speed: 200., base }
    }

    fn process(&mut self, delta: f64) {
        let player = self
            .base_mut()
            .get_tree()
            .expect("we must have a scene tree")
            .get_current_scene()
            .expect("must have a current scene")
            .get_node_as::<CharacterBody2D>("Player");

        let player_position = player.get_global_position();
        let enemy_position = self.base().get_global_position();
        let new_position = enemy_position.move_toward(player_position, self.speed as f32 * delta as f32);
        self.base_mut().set_global_position(new_position);
    }
}
