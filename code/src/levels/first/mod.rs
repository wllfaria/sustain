use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct LevelData {
    #[export]
    pub waves: Array<Variant>,
    pub base: Base<Resource>,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct WaveData {
    #[export]
    pub monsters: Array<Variant>,
    pub base: Base<Resource>,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct WaveMonsterData {
    #[export]
    pub amount: u32,
    #[export]
    pub kind: u32,
    pub base: Base<Resource>,
}
