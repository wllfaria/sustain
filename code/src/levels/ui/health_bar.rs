use godot::classes::{ITextureProgressBar, TextureProgressBar};
use godot::prelude::*;

use crate::entities::player::Player;

#[derive(GodotClass)]
#[class(base=TextureProgressBar)]
pub struct HealthBar {
    #[export]
    player: Option<Gd<Player>>,
    base: Base<TextureProgressBar>,
}

#[godot_api]
impl ITextureProgressBar for HealthBar {
    fn init(base: Base<TextureProgressBar>) -> Self {
        Self { base, player: None }
    }

    fn process(&mut self, _: f64) {
        let player = self.player.as_ref().expect("health bar must have a player attached");
        let hp = player.bind().get_hp().expect("player must have a hp component");

        self.base_mut().set_value(hp.bind().get_current());
    }
}

#[godot_api]
impl HealthBar {
    #[func]
    pub fn set_player_inner(&mut self, player: Option<Gd<Player>>) {
        self.player = player;

        if let Some(player) = &self.player {
            let hp = player.bind().get_hp().expect("player must have a hp component");
            self.base_mut().set_max(hp.bind().get_max());
        }
    }
}
