use raylib::prelude::*;

pub fn draw_victory(d: &mut RaylibDrawHandle, rl: &RaylibHandle) -> bool {
    d.draw_text("🎉 ¡Ganaste! 🎉", 250, 200, 40, Color::GREEN);
    d.draw_text("Presiona ENTER para volver al menú", 180, 300, 30, Color::WHITE);

    rl.is_key_pressed(KeyboardKey::KEY_ENTER)
}
