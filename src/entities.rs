use crate::math::Vec2;

#[derive(Clone, Copy)]
pub struct Bullet { pub p: Vec2, pub v: Vec2, pub r: f32, pub life: f32, pub alive: bool }

#[derive(Clone, Copy)]
pub struct Enemy  { pub p: Vec2, pub v: Vec2, pub r: f32, pub speed: f32, pub hp: i32, pub alive: bool }

pub struct Player {
    pub p: Vec2,
    pub r: f32,
    pub speed: f32,
    pub hp: i32,
    pub shoot_cd: f32,
    pub shoot_t: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            p: Vec2 { x: (crate::config::WIDTH as f32)*0.5, y: (crate::config::HEIGHT as f32)*0.5 },
            r: 12.0, speed: 300.0, hp: 100, shoot_cd: 0.12, shoot_t: 0.0
        }
    }
}
