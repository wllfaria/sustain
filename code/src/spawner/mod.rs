mod fixed_spawner_strategy;

use std::ops::{Deref, DerefMut};

use fixed_spawner_strategy::{ExponentialSpawnerStrategy, FixedSpawnerStrategy};
use godot::classes::{DisplayServer, Timer};
use godot::global::randf_range;
use godot::prelude::*;

use crate::entities::player::Player;

pub trait SpawnerStrategy {
    fn spawn_wave(&mut self, player: Gd<Player>, container: &mut Gd<Node>, bundle: WaveBundle);
}

struct SpawnerStrategyGodot {inner: Box<dyn SpawnerStrategy> };

impl Deref for SpawnerStrategyGodot {
    type Target = dyn SpawnerStrategy;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl DerefMut for SpawnerStrategyGodot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

#[derive(Debug)]
pub struct WaveBundle {
    pub enemy_pool: Vec<Gd<PackedScene>>,
    pub wave: u32,
}

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
    #[export]
    spawn_strategy: Option<Gd<SpawnerStrategyGodot>>,
    #[export]
    current_wave: u32,
    #[export]
    max_wave: u32,
    enemy: Gd<PackedScene>,
}

#[godot_api]
impl INode for Spawner {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            player: None,
            wave_timer: None,
            current_wave: 1,
            max_wave: 20,
            spawn_node: None,
            spawn_strategy: None,
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
        let player = self.player.as_ref().expect("spawner must have a player");
        let container = self
            .spawn_node
            .as_mut()
            .expect("spawner must have a node to spawn enemies");

        let strategy = self.spawn_strategy.as_mut().expect("spawner must have a strategy");

        let bundle = WaveBundle {
            enemy_pool: vec![self.enemy.clone()],
            wave: self.current_wave,
        };

        if let Ok(mut strategy) = strategy.clone().try_cast::<FixedSpawnerStrategy>() {
            strategy.bind_mut().spawn_wave(player.clone(), container, bundle);
        } else if let Ok(mut strategy) = strategy.clone().try_cast::<ExponentialSpawnerStrategy>() {
            strategy.bind_mut().spawn_wave(player.clone(), container, bundle);
        };

        self.wave_timer.as_mut().expect("spawner must have timer").start();
    }
}

pub fn make_spawn_position(player: GdRef<'_, Player>) -> Vector2 {
    let position = player.base().get_global_position();
    let window_size = DisplayServer::singleton().window_get_size();

    let viewport = Rect2::new(
        Vector2::new(
            position.x - (window_size.x / 2) as f32,
            position.y - (window_size.y / 2) as f32,
        ),
        Vector2::new(window_size.x as f32, window_size.y as f32),
    );

    make_bounded_position(viewport, 160., 90.)
}

pub fn make_bounded_position(viewport: Rect2, padding_x: f64, padding_y: f64) -> Vector2 {
    let mut position = Vector2::new(
        randf_range(
            viewport.position.x as f64 - padding_x,
            viewport.position.x as f64 + viewport.size.x as f64 - padding_x,
        ) as f32,
        randf_range(
            viewport.position.y as f64 - padding_y,
            viewport.position.y as f64 + viewport.size.y as f64 + padding_y,
        ) as f32,
    );

    while viewport.has_point(position) {
        position = make_bounded_position(viewport, 160., 90.);
    }

    position
}
