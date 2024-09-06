use raylib::{color::Color, ffi::{KeyboardKey, Vector2}, prelude::RaylibDraw, RaylibHandle, RaylibThread};

struct Player {
    position: Vector2,
    size: Vector2,
    color: Color
}

struct Game {
    player: Player
}

impl Game {
    fn default() -> Game {
        Game{player: Player { position: Vector2 { x: 400.0, y: 400.0 }, size: Vector2 { x: 100.0, y: 50.0 }, color: Color::RED }}
    }
}

fn main() {
    // initialize raylib and create a window
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Crusty")
        .build();
    rl.set_target_fps(250);
    let mut game = Game::default();
    while !rl.window_should_close() {
        // handle keyboard input for our player
        update_game(&mut rl, &mut game);
        draw_game(&mut rl, &thread, &mut game);
    }
}

fn update_game(rl: &mut RaylibHandle, game: &mut Game) {
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        game.player.position.y -= 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        game.player.position.y += 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        game.player.position.x -= 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        game.player.position.x += 1.0;
    }
}

fn draw_game(rl: &mut RaylibHandle, thread: &RaylibThread, game: &mut Game) {

    let mut draw = rl.begin_drawing(thread);
    draw.draw_fps(700, 500);
    draw.clear_background(Color::PURPLE);
    draw.draw_text("Hello Crusty", 350, 300, 20, Color::WHITE);
    //draw our player
    draw.draw_rectangle_v(game.player.position, game.player.size, game.player.color);

}
