use raylib::prelude::*;

pub fn draw_menu(d: &mut RaylibDrawHandle, rl: &RaylibHandle) -> Option<usize> {
    d.draw_text("🐱 Cat Maze - Raycaster", 180, 100, 40, Color::YELLOW);
    d.draw_text("Presiona 1 para Nivel 1", 250, 250, 30, Color::WHITE);
    d.draw_text("Presiona 2 para Nivel 2", 250, 300, 30, Color::WHITE);

    if rl.is_key_pressed(KeyboardKey::KEY_ONE) { return Some(0); }
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) { return Some(1); }
    None
}
