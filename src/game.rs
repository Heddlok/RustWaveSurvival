use rand::prelude::*;

use crate::config::{WIDTH, HEIGHT};
use crate::entities::{Bullet, Enemy, Player};
use crate::input::Input;
use crate::math::{clamp_rect, Vec2};
use crate::render::{fill_circle, fill_rect, rgb};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Phase { Intermission, Combat, GameOver }

pub struct Game {
    rng: StdRng,
    pub player: Player,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub wave: i32,
    pub enemies_alive: i32,
    pub score: i32,
    pub phase: Phase,
    pub intermission: f32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            rng: StdRng::from_entropy(),
            player: Player::default(),
            bullets: Vec::with_capacity(512),
            enemies: Vec::with_capacity(512),
            wave: 0, enemies_alive: 0, score: 0,
            phase: Phase::Intermission,
            intermission: 2.0
        }
    }
}

impl Game {
    pub fn reset(&mut self) { *self = Game::default(); }

    pub fn begin_wave(&mut self) {
        self.wave += 1;
        let base = 6; let per = 4;
        let to_spawn = base + per * self.wave as usize;
    
        self.enemies.clear();
        self.enemies.reserve(to_spawn);
    
        for _ in 0..to_spawn {
            let e = self.make_enemy();    // borrow self here
            self.enemies.push(e);         // borrow self.enemies here (non-overlapping)
        }
    
        self.enemies_alive = to_spawn as i32;
        self.phase = Phase::Combat;
    }
    

    fn make_enemy(&mut self) -> Enemy {
        let margin = 30.0f32;
        let edge = self.rng.gen_range(0..4);
        let p = match edge {
            0 => Vec2 { x: self.rng.gen_range(0.0..WIDTH as f32), y: -margin },
            1 => Vec2 { x: WIDTH as f32 + margin, y: self.rng.gen_range(0.0..HEIGHT as f32) },
            2 => Vec2 { x: self.rng.gen_range(0.0..WIDTH as f32), y: HEIGHT as f32 + margin },
            _ => Vec2 { x: -margin, y: self.rng.gen_range(0.0..HEIGHT as f32) },
        };
        let wv = self.wave.max(1) as f32;
        Enemy { p, v: Vec2::default(), r: 14.0, speed: 70.0 + 7.0 * wv, hp: 2 + (self.wave/2) as i32, alive: true }
    }

    pub fn update(&mut self, dt: f32, input: &Input) {
        match self.phase {
            Phase::GameOver => {
                if input.key_enter { self.reset(); }
            }
            Phase::Intermission => {
                self.update_player(dt, input);
                self.update_bullets(dt);   // <-- keep bullets moving
                self.cull_dead();          // <-- remove expired bullets
                self.intermission -= dt;
                if self.intermission <= 0.0 { self.begin_wave(); }
            }
            Phase::Combat => {
                self.update_player(dt, input);
                self.shoot_if_needed(dt, input);
                self.update_bullets(dt);
                self.update_enemies(dt);
                self.bullet_enemy_collisions();
                self.cull_dead();
                if self.enemies_alive <= 0 { self.phase = Phase::Intermission; self.intermission = 3.0; }
            }
        }
    }
    

    fn update_player(&mut self, dt: f32, input: &Input) {
        let mut d = Vec2 { x: 0.0, y: 0.0 };
        if input.w { d.y -= 1.0; }
        if input.s { d.y += 1.0; }
        if input.a { d.x -= 1.0; }
        if input.d { d.x += 1.0; }
        d = d.norm();
        self.player.p = clamp_rect(self.player.p.add(d.mul(self.player.speed * dt)), self.player.r);

        for e in self.enemies.iter_mut() {
            if !e.alive { continue; }
            let rr = e.r + self.player.r;
            let diff = e.p.sub(self.player.p);
            if diff.len2() <= rr * rr {
                let dir = diff.norm();
                self.player.hp -= 10;
                e.p = e.p.add(dir.mul(8.0));
                if self.player.hp <= 0 { self.phase = Phase::GameOver; }
            }
        }
    }

    fn shoot_if_needed(&mut self, dt: f32, input: &Input) {
        self.player.shoot_t -= dt;
        if !input.mouse_left || self.player.shoot_t > 0.0 { return; }
        let dir = Vec2 { x: input.mouse_pos.x - self.player.p.x, y: input.mouse_pos.y - self.player.p.y }.norm();
        if dir.len2() < 1e-6 { return; }
        let b = Bullet { p: self.player.p.add(dir.mul(self.player.r + 6.0)), v: dir.mul(800.0), r: 4.0, life: 2.0, alive: true };
        self.bullets.push(b);
        self.player.shoot_t = self.player.shoot_cd;
    }

    fn update_bullets(&mut self, dt: f32) {
        for b in self.bullets.iter_mut() {
            if !b.alive { continue; }
            b.p = b.p.add(b.v.mul(dt));
            b.life -= dt;
            if b.life <= 0.0 || b.p.x < -50.0 || b.p.x > WIDTH as f32 + 50.0 || b.p.y < -50.0 || b.p.y > HEIGHT as f32 + 50.0 {
                b.alive = false;
            }
        }
    }

    fn update_enemies(&mut self, dt: f32) {
        for e in self.enemies.iter_mut() {
            if !e.alive { continue; }
            let dir = Vec2 { x: self.player.p.x - e.p.x, y: self.player.p.y - e.p.y }.norm();
            e.v = dir.mul(e.speed);
            e.p = e.p.add(e.v.mul(dt));
        }
    }

    fn bullet_enemy_collisions(&mut self) {
        for e in self.enemies.iter_mut() {
            if !e.alive { continue; }
            for b in self.bullets.iter_mut() {
                if !b.alive { continue; }
                let rr = e.r + b.r;
                let diff = e.p.sub(b.p);
                if diff.len2() <= rr * rr {
                    b.alive = false;
                    e.hp -= 1;
                    if e.hp <= 0 { e.alive = false; self.enemies_alive -= 1; self.score += 10; }
                }
            }
        }
    }

    fn cull_dead(&mut self) {
        self.bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.alive);
    }

    pub fn draw(&self, fb: &mut [u32]) {
        fill_rect(fb, 0, 0, WIDTH as i32, HEIGHT as i32, rgb(11,14,18));
        for b in &self.bullets { if b.alive { fill_circle(fb, b.p.x as i32, b.p.y as i32, b.r as i32, rgb(90,200,255)); } }
        for e in &self.enemies { if e.alive { fill_circle(fb, e.p.x as i32, e.p.y as i32, e.r as i32, rgb(230,76,76)); } }
        fill_circle(fb, self.player.p.x as i32, self.player.p.y as i32, self.player.r as i32, 0x00F0F0F0);

        let hpw = ((self.player.hp.max(0).min(100) as f32 / 100.0) * (WIDTH as f32 * 0.3)) as i32;
        fill_rect(fb, 20, 20, hpw, 10, rgb(76, 230, 140));
    }

    pub fn window_title(&self) -> String {
        format!(
            "Wave {} | Enemies {} | HP {} | Score {}{}",
            self.wave,
            self.enemies_alive,
            self.player.hp.max(0),
            self.score,
            match self.phase {
                Phase::Intermission => " | Intermission",
                Phase::Combat => "",
                Phase::GameOver => " | GAME OVER (Enter to reset)",
            }
        )
    }
}
