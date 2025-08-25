use raylib::prelude::*;
use crate::map::{MAP_W, MAP_H};
use crate::player::Player;

pub fn render_world(
    d: &mut RaylibDrawHandle,
    screen_w: i32,
    screen_h: i32,
    player: &Player,
    map: &[[i32; MAP_W]; MAP_H],
    tex_wall1: &Texture2D,
    tex_wall2: &Texture2D,
    tex_wall3: &Texture2D,
) {
    let (dir_x, dir_y) = player.dir_vec();
    let (plane_x, plane_y) = player.plane_vec();

    for x in 0..screen_w {
        let camera_x = 2.0 * x as f32 / screen_w as f32 - 1.0;
        let ray_dir_x = dir_x + plane_x * camera_x;
        let ray_dir_y = dir_y + plane_y * camera_x;

        let mut map_x = player.x as i32;
        let mut map_y = player.y as i32;

        let delta_dist_x = if ray_dir_x == 0.0 { f32::INFINITY } else { (1.0 / ray_dir_x).abs() };
        let delta_dist_y = if ray_dir_y == 0.0 { f32::INFINITY } else { (1.0 / ray_dir_y).abs() };

        let (mut step_x, mut step_y);
        let (mut side_dist_x, mut side_dist_y);

        if ray_dir_x < 0.0 {
            step_x = -1;
            side_dist_x = (player.x - map_x as f32) * delta_dist_x;
        } else {
            step_x = 1;
            side_dist_x = (map_x as f32 + 1.0 - player.x) * delta_dist_x;
        }
        if ray_dir_y < 0.0 {
            step_y = -1;
            side_dist_y = (player.y - map_y as f32) * delta_dist_y;
        } else {
            step_y = 1;
            side_dist_y = (map_y as f32 + 1.0 - player.y) * delta_dist_y;
        }

        // DDA
        let mut hit = false;
        let mut side = 0;
        while !hit {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }
            if map_x < 0 || map_y < 0 || map_x >= MAP_W as i32 || map_y >= MAP_H as i32 {
                break;
            }
            if map[map_y as usize][map_x as usize] > 0 {
                hit = true;
            }
        }
        if !hit { continue; }

        // Distancia perpendicular
        let perp_wall_dist = if side == 0 {
            (map_x as f32 - player.x + (1 - step_x) as f32 * 0.5) / ray_dir_x
        } else {
            (map_y as f32 - player.y + (1 - step_y) as f32 * 0.5) / ray_dir_y
        };
        if perp_wall_dist <= 0.0 { continue; }

        // Altura de la pared
        let line_h = (screen_h as f32 / perp_wall_dist) as i32;
        let mut draw_start = -line_h / 2 + screen_h / 2;
        let mut draw_end   =  line_h / 2 + screen_h / 2;
        if draw_start < 0 { draw_start = 0; }
        if draw_end >= screen_h { draw_end = screen_h - 1; }

        // Tipo de pared → textura o meta
        let wall_type = map[map_y as usize][map_x as usize];

        if wall_type == 9 {
            // META → siempre visible
            d.draw_line(x, draw_start, x, draw_end, Color::YELLOW);
        } else {
            // Selección de textura por tipo
            let tex = match wall_type {
                1 => tex_wall1,
                2 => tex_wall2,
                3 => tex_wall3,
                _ => tex_wall1,
            };

            // Posición exacta donde golpea el rayo
            let wall_x = if side == 0 {
                player.y + perp_wall_dist * ray_dir_y
            } else {
                player.x + perp_wall_dist * ray_dir_x
            };
            let wall_x = wall_x - wall_x.floor();

            // Coordenada X en la textura
            let tex_x = (wall_x * tex.width() as f32) as i32;
            let tex_x = if side == 0 && ray_dir_x > 0.0 {
                tex.width() - tex_x - 1
            } else if side == 1 && ray_dir_y < 0.0 {
                tex.width() - tex_x - 1
            } else {
                tex_x
            };

            // Dibujar la textura estirada como una línea vertical
            d.draw_texture_pro(
                tex,
                Rectangle::new(tex_x as f32, 0.0, 1.0, tex.height() as f32),
                Rectangle::new(x as f32, draw_start as f32, 1.0, line_h as f32),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    }
}
