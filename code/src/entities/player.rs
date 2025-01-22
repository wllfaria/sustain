use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { speed: 400.0, base }
    }

    fn process(&mut self, delta: f64) {
        self.get_input(delta)
    }
}

impl Player {
    fn get_input(&mut self, delta: f64) {
        let direction = Input::singleton().get_vector("left", "right", "up", "down");
        if Vector2::ZERO == direction {
            return;
        }

        let speed = self.speed as f32;
        let direction = direction.normalized();

        let mut base = self.base_mut();

        let new_position = base.get_global_position() + direction * speed * delta as f32;
        base.set_global_position(new_position);
    }
}
