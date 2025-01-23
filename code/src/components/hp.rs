use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct HpComponent {
    #[export]
    entity: Option<Gd<Node2D>>,
    #[export]
    current: f64,
    #[export]
    max: f64,
    base: Base<Node2D>,
}

#[godot_api]
impl HpComponent {
    #[func]
    pub fn take_damage(&mut self, damage: f64) {
        self.current -= damage;
    }

    #[func]
    pub fn is_dead(&self) -> bool {
        self.current == 0.
    }
}
