use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::entities::player::Player;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct MeleeEnemy {
    #[export]
    speed: f64,
    #[export]
    player: Option<Gd<Player>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for MeleeEnemy {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            speed: 200.,
            player: None,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let player = self.player.as_ref().expect("enemies must have a player to follow");
        let player_position = player.get_global_position();
        let enemy_position = self.base().get_global_position();
        let new_position = enemy_position.move_toward(player_position, self.speed as f32 * delta as f32);
        self.base_mut().set_global_position(new_position);
    }
}
