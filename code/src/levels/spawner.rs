use godot::classes::Timer;
use godot::global::randf_range;
use godot::prelude::*;

use crate::entities::enemies::melee::MeleeEnemy;
use crate::entities::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
struct Spawner {
    base: Base<Node>,
    #[export]
    player: Option<Gd<Player>>,
    #[export]
    wave_timer: Option<Gd<Timer>>,
    #[export]
    spawn_node: Option<Gd<Node>>,
    enemy: Gd<PackedScene>,
}

#[godot_api]
impl INode for Spawner {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            player: None,
            wave_timer: None,
            spawn_node: None,
            enemy: load::<PackedScene>("res://entities/enemies/melee/MeleeEnemy.tscn"),
        }
    }

    fn ready(&mut self) {
        let callable = self.base_mut().callable("on_spawn_timer_timeout");

        self.wave_timer
            .as_mut()
            .expect("spawner must have a cooldown timer")
            .connect("timeout", &callable);
    }
}

#[godot_api]
impl Spawner {
    #[func]
    fn on_spawn_timer_timeout(&mut self) {
        let timer = self.wave_timer.as_mut().expect("spawner must have timer");
        let spawn_node = self
            .spawn_node
            .as_mut()
            .expect("spawner must have a node to spawn enemies");

        let make_pos = || Vector2::new(randf_range(-160., 480.) as f32, randf_range(-90., 270.) as f32);

        let mut enemy_position = make_pos();

        while enemy_position.x < 440. && enemy_position.x > -80. && enemy_position.y < 230. && enemy_position.y > -45. {
            enemy_position = make_pos();
        }

        let mut node = self.enemy.instantiate_as::<MeleeEnemy>();
        node.bind_mut().set_player(self.player.clone());
        node.set_global_position(enemy_position);

        spawn_node.add_child(&node);

        timer.start();
    }
}
