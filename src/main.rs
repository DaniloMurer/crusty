use raylib::{color::Color, ffi::{KeyboardKey, Vector2}, prelude::RaylibDraw, RaylibHandle, RaylibThread};

struct Player {
    position: Vector2,
    size: Vector2,
    color: Color
}

struct Game {
    player: Player,
    projectile: Projectile
}

struct Projectile {
    position: Vector2,
    force: Vector2,
    radius: f32,
    should_render: bool,
    color: Color
}

impl Game {
    fn default() -> Game {
        let player = Player {
            position: Vector2 { x: 400.0, y: 400.0 },
            size: Vector2 { x: 100.0, y: 50.0 },
            color: Color::RED
        };
        let projectile = Projectile {
            position: Vector2 { x: player.position.x, y: player.position.y - 20.0 },
            force: Vector2 { x: 0.0, y: 5.0 },
            radius: 0.0,
            should_render: false,
            color: Color::BLUE
        };
        Game {
            player,
            projectile
        }
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
    } else if rl.is_key_down(KeyboardKey::KEY_SPACE) {
        game.projectile.should_render = true;
        game.projectile.position.y -= game.projectile.force.y;
        game.projectile.position.x = game.player.position.x;
        game.projectile.radius = 20.0;
    }
}

fn draw_game(rl: &mut RaylibHandle, thread: &RaylibThread, game: &mut Game) {

    let mut draw = rl.begin_drawing(thread);
    draw.draw_fps(700, 500);
    draw.clear_background(Color::PURPLE);
    draw.draw_text("Hello Crusty", 350, 300, 20, Color::WHITE);
    //draw our player
    draw.draw_rectangle_v(game.player.position, game.player.size, game.player.color);
    if game.projectile.should_render {
        draw.draw_circle_v(game.projectile.position, game.projectile.radius, game.projectile.color);
    }
}
