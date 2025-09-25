use crate::math::Vec2;

#[derive(Default, Clone, Copy)]
pub struct Input {
    pub w: bool, pub a: bool, pub s: bool, pub d: bool,
    pub key_enter: bool,
    pub mouse_left: bool,
    pub mouse_pos: Vec2,
}
