use godot::prelude::*;

use super::{make_spawn_position, SpawnerStrategy, WaveBundle};
use crate::entities::enemies::melee::MeleeEnemy;
use crate::entities::player::Player;

#[derive(Debug, GodotClass)]
#[class(init, base=Node)]
pub struct FixedSpawnerStrategy {
    base: Base<Node>,
}

impl SpawnerStrategy for FixedSpawnerStrategy {
    fn spawn_wave(&mut self, player: Gd<Player>, container: &mut Gd<Node>, bundle: WaveBundle) {
        for _ in 0..20 {
            let enemy_position = make_spawn_position(player.bind());
            let mut node = bundle.enemy_pool[0].instantiate_as::<MeleeEnemy>();
            node.bind_mut().set_player(Some(player.clone()));
            node.set_global_position(enemy_position);
            container.add_child(&node);
        }
    }
}

#[derive(Debug, GodotClass)]
#[class(init, base=Node)]
pub struct ExponentialSpawnerStrategy {}

impl SpawnerStrategy for ExponentialSpawnerStrategy {
    fn spawn_wave(&mut self, player: Gd<Player>, container: &mut Gd<Node>, bundle: WaveBundle) {}
}
