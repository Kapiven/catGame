use raylib::prelude::*;
use crate::map::{MAP_W, MAP_H};
use crate::player::Player;
use crate::map::tile_color;

pub fn draw_minimap(
    d: &mut RaylibDrawHandle,
    player: &Player,
    map: &[[i32; MAP_W]; MAP_H],
) {
    let map_scale: i32 = 14;
    let ox: i32 = 16;
    let oy: i32 = 16;
    let mm_w = (MAP_W as i32) * map_scale;
    let mm_h = (MAP_H as i32) * map_scale;

    d.draw_rectangle(ox - 6, oy - 6, mm_w + 12, mm_h + 12, Color::new(0,0,0,160));
    d.draw_rectangle_lines(ox - 6, oy - 6, mm_w + 12, mm_h + 12, Color::WHITE);

    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let t = map[y][x];
            let col = if t == 0 { Color::LIGHTGRAY } else { tile_color(t) };
            d.draw_rectangle(
                ox + (x as i32 * map_scale),
                oy + (y as i32 * map_scale),
                map_scale,
                map_scale,
                col
            );
        }
    }

    // jugador
    let px = ox + (player.x * map_scale as f32) as i32;
    let py = oy + (player.y * map_scale as f32) as i32;
    d.draw_circle(px, py, 3.0, Color::RED);

    // dirección
    let dir_len = 12.0;
    let fx = px as f32 + player.dir.cos() * dir_len;
    let fy = py as f32 + player.dir.sin() * dir_len;
    d.draw_line(px, py, fx as i32, fy as i32, Color::RED);
}

pub fn draw_hud(d: &mut RaylibDrawHandle, screen_w: i32) {
    d.draw_fps(screen_w - 90, 10);
    d.draw_text("W/S: mover | A/D o Mouse: girar", 12, 12, 20, Color::WHITE);
}
