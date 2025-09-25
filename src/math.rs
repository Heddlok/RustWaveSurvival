#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2 { pub x: f32, pub y: f32 }

impl Vec2 {
    pub fn add(self, o: Vec2) -> Vec2 { Vec2 { x: self.x + o.x, y: self.y + o.y } }
    pub fn sub(self, o: Vec2) -> Vec2 { Vec2 { x: self.x - o.x, y: self.y - o.y } }
    pub fn mul(self, s: f32) -> Vec2 { Vec2 { x: self.x * s, y: self.y * s } }
    pub fn len2(self) -> f32 { self.x*self.x + self.y*self.y }
    pub fn len(self) -> f32 { self.len2().sqrt() }
    pub fn norm(self) -> Vec2 { let l=self.len(); if l>1e-6 { self.mul(1.0/l) } else { Vec2::default() } }
}

use crate::config::{WIDTH, HEIGHT};
pub fn clamp_rect(mut p: Vec2, r: f32) -> Vec2 {
    if p.x < r { p.x = r }
    if p.y < r { p.y = r }
    if p.x > (WIDTH as f32 - r) { p.x = WIDTH as f32 - r }
    if p.y > (HEIGHT as f32 - r) { p.y = HEIGHT as f32 - r }
    p
}
