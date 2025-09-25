use crate::config::{WIDTH, HEIGHT};

#[inline]
pub fn rgb(r: u8, g: u8, b: u8) -> u32 { ((r as u32) << 16) | ((g as u32) << 8) | (b as u32) }

#[inline]
pub fn put_px(fb: &mut [u32], x: i32, y: i32, c: u32) {
    if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 { return; }
    fb[y as usize * WIDTH + x as usize] = c;
}

pub fn fill_rect(fb: &mut [u32], x: i32, y: i32, w: i32, h: i32, c: u32) {
    let x0 = x.max(0); let y0 = y.max(0);
    let x1 = (x + w).min(WIDTH as i32); let y1 = (y + h).min(HEIGHT as i32);
    for yy in y0..y1 { for xx in x0..x1 { put_px(fb, xx, yy, c); } }
}

pub fn fill_circle(fb: &mut [u32], cx: i32, cy: i32, r: i32, c: u32) {
    if r <= 0 { return; }
    let r2 = (r * r) as i32;
    for y in -r..=r {
        let yy = y * y;
        let w = (r2 - yy).max(0) as f32;
        let w = w.sqrt() as i32;
        for x in -w..=w { put_px(fb, cx + x, cy + y, c); }
    }
}
