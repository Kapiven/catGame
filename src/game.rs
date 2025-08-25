use raylib::prelude::*;

pub const MAP_W: usize = 8;
pub const MAP_H: usize = 8;

pub const MAP1: [[i32; MAP_W]; MAP_H] = [
    [1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,1],
    [1,0,2,0,0,3,0,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,1,0,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,9,1],
    [1,1,1,1,1,1,1,1],
];

pub const MAP2: [[i32; MAP_W]; MAP_H] = [
    [1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,1],
    [1,0,3,0,2,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,1,0,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,9,1],
    [1,1,1,1,1,1,1,1],
];

pub const MAPS: [&[[i32; MAP_W]; MAP_H]; 2] = [&MAP1, &MAP2];

#[derive(Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dir: f32,
}

pub struct Game {
    pub player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player { x: 3.5, y: 3.5, dir: 0.0 },
        }
    }

    pub fn start(&mut self, _level: usize) {
        self.player.x = 3.5;
        self.player.y = 3.5;
        self.player.dir = 0.0;
    }

    pub fn update(
        &mut self,
        d: &mut RaylibDrawHandle,
        rl: &RaylibHandle,
        level: usize,
        music: &mut Music,
        sfx_step: &Sound,
    ) -> bool {
        let map = MAPS[level];

        // 🎵 Mantener música
        music.update_stream();

        // 🔹 Controles
        if rl.is_key_down(KeyboardKey::KEY_A) { self.player.dir -= 0.05; }
        if rl.is_key_down(KeyboardKey::KEY_D) { self.player.dir += 0.05; }

        let md = rl.get_mouse_delta();
        self.player.dir += md.x * 0.003;

        let move_step = if rl.is_key_down(KeyboardKey::KEY_W) { 0.08 }
            else if rl.is_key_down(KeyboardKey::KEY_S) { -0.08 }
            else { 0.0 };

        if move_step != 0.0 { sfx_step.play(); }

        let try_x = self.player.x + move_step * self.player.dir.cos();
        let try_y = self.player.y + move_step * self.player.dir.sin();

        if self.is_walkable(try_x, self.player.y, map) { self.player.x = try_x; }
        if self.is_walkable(self.player.x, try_y, map) { self.player.y = try_y; }

        // 🔹 Dibujar fondo
        d.draw_rectangle(0, 0, 800, 300, Color::SKYBLUE);
        d.draw_rectangle(0, 300, 800, 300, Color::DARKBROWN);

        // 🔹 Raycasting
        self.raycast(d, map);

        // 🔹 Minimap
        self.draw_minimap(d, map);

        d.draw_fps(710, 10);
        d.draw_text("WASD: mover | Mouse: girar | SFX: pasos", 10, 10, 20, Color::WHITE);

        // 🔹 Condición de victoria
        let gx = self.player.x as usize;
        let gy = self.player.y as usize;
        map[gy][gx] == 9
    }

    fn is_walkable(&self, x: f32, y: f32, map: &[[i32; MAP_W]; MAP_H]) -> bool {
        if x < 0.0 || y < 0.0 { return false; }
        let gx = x as usize;
        let gy = y as usize;
        gx < MAP_W && gy < MAP_H && map[gy][gx] != 1 && map[gy][gx] != 2 && map[gy][gx] != 3
    }

    fn draw_minimap(&self, d: &mut RaylibDrawHandle, map: &[[i32; MAP_W]; MAP_H]) {
        let scale = 14;
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                let color = match map[y][x] {
                    0 => Color::LIGHTGRAY,
                    1 => Color::DARKGRAY,
                    2 => Color::ORANGE,
                    3 => Color::PINK,
                    9 => Color::GREEN,
                    _ => Color::WHITE,
                };
                d.draw_rectangle(
                    16 + (x as i32)*scale,
                    16 + (y as i32)*scale,
                    scale,
                    scale,
                    color,
                );
            }
        }

        let px = 16 + (self.player.x * scale as f32) as i32;
        let py = 16 + (self.player.y * scale as f32) as i32;
        d.draw_circle(px, py, 3.0, Color::RED);

        let fx = px as f32 + self.player.dir.cos()*12.0;
        let fy = py as f32 + self.player.dir.sin()*12.0;
        d.draw_line(px, py, fx as i32, fy as i32, Color::RED);
    }

    fn raycast(&self, d: &mut RaylibDrawHandle, map: &[[i32; MAP_W]; MAP_H]) {
        for x in 0..800 {
            let camera_x = 2.0*x as f32/800.0 - 1.0;
            let plane_x = -self.player.dir.sin()*0.66;
            let plane_y = self.player.dir.cos()*0.66;
            let ray_dir_x = self.player.dir.cos() + plane_x*camera_x;
            let ray_dir_y = self.player.dir.sin() + plane_y*camera_x;

            let mut map_x = self.player.x as i32;
            let mut map_y = self.player.y as i32;

            let delta_dist_x = if ray_dir_x==0.0 { f32::INFINITY } else { (1.0/ray_dir_x).abs() };
            let delta_dist_y = if ray_dir_y==0.0 { f32::INFINITY } else { (1.0/ray_dir_y).abs() };

            let (mut side_dist_x, mut side_dist_y, step_x, step_y);
            if ray_dir_x <0.0 { step_x=-1; side_dist_x=(self.player.x-map_x as f32)*delta_dist_x; }
            else { step_x=1; side_dist_x=(map_x as f32+1.0-self.player.x)*delta_dist_x; }
            if ray_dir_y <0.0 { step_y=-1; side_dist_y=(self.player.y-map_y as f32)*delta_dist_y; }
            else { step_y=1; side_dist_y=(map_y as f32+1.0-self.player.y)*delta_dist_y; }

            let mut hit=0; let mut side=0;
            while hit==0 {
                if side_dist_x<side_dist_y { side_dist_x+=delta_dist_x; map_x+=step_x; side=0; }
                else { side_dist_y+=delta_dist_y; map_y+=step_y; side=1; }
                if map_x<0 || map_y<0 || map_x>=MAP_W as i32 || map_y>=MAP_H as i32 { break; }
                if map[map_y as usize][map_x as usize]>0 { hit=1; }
            }
            if hit==0 { continue; }

            let perp_wall_dist = if side==0 {
                (map_x as f32 - self.player.x + (1-step_x) as f32/2.0)/ray_dir_x
            } else {
                (map_y as f32 - self.player.y + (1-step_y) as f32/2.0)/ray_dir_y
            };
            if perp_wall_dist<=0.0 { continue; }
            let line_height = (600.0/perp_wall_dist) as i32;
            let mut draw_start=-line_height/2+300;
            let mut draw_end=line_height/2+300;
            if draw_start<0 { draw_start=0; }
            if draw_end>=600 { draw_end=599; }

            let wall_type = map[map_y as usize][map_x as usize];
            let color = match wall_type {
                1=>Color::GRAY, 2=>Color::ORANGE, 3=>Color::PINK, 9=>Color::GREEN, _=>Color::WHITE,
            };
            let final_color = if side==1 { Color::fade(&color,0.7) } else { color };
            d.draw_line(x, draw_start, x, draw_end, final_color);
        }
    }
}
