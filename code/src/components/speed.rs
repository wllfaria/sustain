use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct SpeedComponent {
    #[export]
    speed: f64,
    #[export]
    entity: Option<Gd<Node2D>>,
    base: Base<Node2D>,
}
