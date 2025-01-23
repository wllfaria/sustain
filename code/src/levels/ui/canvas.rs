use godot::classes::{CanvasLayer, ICanvasLayer};
use godot::prelude::*;

use super::health_bar::HealthBar;
use crate::entities::player::Player;

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct Canvas {
    #[export]
    player: Option<Gd<Player>>,
    #[export]
    health_bar: Option<Gd<HealthBar>>,
    base: Base<CanvasLayer>,
}

#[godot_api]
impl ICanvasLayer for Canvas {
    fn ready(&mut self) {
        let health_bar = self
            .health_bar
            .as_mut()
            .expect("UI layer must have a health bar reference");

        let player = self.player.clone();

        health_bar.bind_mut().set_player(player)
    }
}
