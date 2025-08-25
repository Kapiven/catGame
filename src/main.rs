use raylib::prelude::*;
use raylib::core::audio::RaylibAudio;

mod map;
mod player;
mod render;
mod hud;

use map::{MAP_W, MAP_H, MAP1, MAP2, is_walkable};
use player::Player;
use render::render_world;
use hud::{draw_minimap, draw_hud};


#[derive(PartialEq)]
enum GameState {
    Welcome,
    Playing,
    Success,
}

fn main() {
    const SCREEN_W: i32 = 1000;
    const SCREEN_H: i32 = 800;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title("Jueguito de Gato")
        .build();

    rl.set_target_fps(60);

    // Inicializar audio
    let audio = RaylibAudio::init_audio_device().expect("No se pudo iniciar audio");
    let mut music = audio.new_music("assets/magestic_dreams.ogg").expect("No se pudo cargar música");
    music.set_volume(0.5);
    music.play_stream();

    let sfx_step = audio.new_sound("assets/steps.wav").expect("No se pudo cargar SFX");

    // Cargar texturas de paredes
    let tex_wall1 = rl.load_texture(&thread, "assets/ForestTrees.png").expect("No se pudo cargar ForestTrees.png");
    let tex_wall2 = rl.load_texture(&thread, "assets/ForestTrees.png").expect("No se pudo cargar ForestTrees.png");
    let tex_wall3 = rl.load_texture(&thread, "assets/ForestTrees.png").expect("No se pudo cargar ForestTrees.png");

    // Estado de juego
    let mut game_state = GameState::Welcome;
    let mut map = MAP1;             
    let mut player = Player::new(3.5, 3.5, 0.0); 

    while !rl.window_should_close() {

        // Actualizar stream de música
        music.update_stream();

        // INPUTS
        let key_one    = rl.is_key_pressed(KeyboardKey::KEY_ONE);
        let key_two    = rl.is_key_pressed(KeyboardKey::KEY_TWO);
        let key_enter  = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
        let k_a        = rl.is_key_down(KeyboardKey::KEY_A);
        let k_d        = rl.is_key_down(KeyboardKey::KEY_D);
        let k_w        = rl.is_key_down(KeyboardKey::KEY_W);
        let k_s        = rl.is_key_down(KeyboardKey::KEY_S);
        let mouse_delta = rl.get_mouse_delta();

        let move_step = if rl.is_key_down(KeyboardKey::KEY_W) {
            0.08
        } else if rl.is_key_down(KeyboardKey::KEY_S) {
            -0.08
        } else {
            0.0
        };

        if move_step != 0.0 {
            sfx_step.play(); //Sonido al moverse
        }

        // LÓGICA
        match game_state {
            GameState::Welcome => {
                if key_one {
                    map = MAP1;
                    player = Player::new(3.5, 3.5, 0.0);
                    game_state = GameState::Playing;
                }
                if key_two {
                    map = MAP2;
                    player = Player::new(3.5, 3.5, 0.0);
                    game_state = GameState::Playing;
                }
            }
            GameState::Success => {
                if key_enter {
                    game_state = GameState::Welcome;
                }
            }
            GameState::Playing => {
                // Rotación 
                if k_a { player.dir -= 0.05; }
                if k_d { player.dir += 0.05; }
                player.dir += mouse_delta.x * 0.003;

                // Movimiento hacia delante/atrás con colisiones
                let move_step = if k_w { 0.08 } else if k_s { -0.08 } else { 0.0 };
                if move_step != 0.0 {
                    let try_x = player.x + move_step * player.dir.cos();
                    let try_y = player.y + move_step * player.dir.sin();
                    if is_walkable(&map, try_x, player.y) { player.x = try_x; }
                    if is_walkable(&map, player.x, try_y) { player.y = try_y; }
                }

                // Condición de victoria (llegar a esquina inferior derecha)
                if player.x > (MAP_W as f32 - 1.5) && player.y > (MAP_H as f32 - 1.5) {
                    game_state = GameState::Success;
                }
            }
        }

        // DIBUJO 
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::STEELBLUE); 

        match game_state {
            GameState::Welcome => {
                d.draw_text("CAT MAZE ^¬-¬^", 320, 280, 42, Color::YELLOW);
                d.draw_text("Presiona 1 para Nivel 1", 350, 370, 24, Color::WHITE);
                d.draw_text("Presiona 2 para Nivel 2", 350, 410, 24, Color::WHITE);
                d.draw_text("W/S: Avanzar/Retroceder  |  A/D o Mouse: Girar", 230, 470, 20, Color::LIGHTGRAY);
            }
            GameState::Success => {
                d.draw_text("¡GANASTE! :D", 360, 320, 44, Color::WHITE);
                d.draw_text("ENTER para volver al menú", 330, 400, 24, Color::WHITE);
            }
            GameState::Playing => {
                // Cielo y piso
                d.draw_rectangle(0, 0, SCREEN_W, SCREEN_H/2, Color::DARKBLUE);
                d.draw_rectangle(0, SCREEN_H/2, SCREEN_W, SCREEN_H/2, Color::FORESTGREEN);

                // Raycasting de paredes
                render_world(&mut d, SCREEN_W, SCREEN_H, &player, &map, &tex_wall1, &tex_wall2, &tex_wall3);

                // HUD
                draw_minimap(&mut d, &player, &map);
                draw_hud(&mut d, SCREEN_W);
            }
        }
    }
}
