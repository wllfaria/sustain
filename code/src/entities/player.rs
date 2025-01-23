use godot::classes::{AnimatedSprite2D, AnimationPlayer, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::components::hp::HpComponent;
use crate::components::speed::SpeedComponent;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: Option<Gd<SpeedComponent>>,
    #[export]
    hp: Option<Gd<HpComponent>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        let sprite = self.base().get_node_as::<AnimatedSprite2D>("PlayerSprite");
        let mut animator = sprite.get_node_as::<AnimationPlayer>("PlayerAnimation");

        animator.set_current_animation("Idle");
        animator.play();
    }

    fn process(&mut self, delta: f64) {
        self.get_input(delta);
    }
}

impl Player {
    fn get_input(&mut self, delta: f64) {
        let direction = Input::singleton().get_vector("left", "right", "up", "down");
        if Vector2::ZERO == direction {
            return;
        }

        let speed = self
            .speed
            .as_ref()
            .expect("player must have a speed component")
            .bind()
            .get_speed() as f32;

        let direction = direction.normalized();
        let mut base = self.base_mut();

        let new_position = base.get_global_position() + direction * speed * delta as f32;
        base.set_global_position(new_position);
    }
}
