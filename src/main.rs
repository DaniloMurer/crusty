use raylib::{ffi::{KeyboardKey, Vector2}, prelude::RaylibDraw, RaylibHandle, RaylibThread};
use raylib::color::Color;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const SCREEN_TITLE: &str = "Crusty";
const MAX_FPS: u32 = 60;
const PLAYER_MOVEMENT_SPEED: f32 = 8.0;

struct Player {
    position: Vector2,
    size: Vector2,
    color: Color
}

struct Game {
    player: Player,
    projectiles: Vec<Projectile>,
    fire_next_projectile: bool
}

struct Projectile {
    position: Vector2,
    force: Vector2,
    radius: f32,
    color: Color
}

impl Game {
    fn default() -> Game {
        let player = Player {
            position: Vector2 { x: 400.0, y: 400.0 },
            size: Vector2 { x: 100.0, y: 50.0 },
            color: Color::RED
        };
        let projectiles = Vec::new();

        Game {
            player,
            projectiles,
            fire_next_projectile: true
        }
    }
}

fn main() {
    // initialize raylib and create a window
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(SCREEN_TITLE)
        .build();
    rl.set_target_fps(MAX_FPS);
    let mut game = Game::default();
    while !rl.window_should_close() {
        update_game(&mut rl, &mut game);
        draw_game(&mut rl, &thread, &mut game);
    }
}

fn update_game(rl: &mut RaylibHandle, game: &mut Game) {
    let mut index_to_delete = 0;
    let mut should_delete = false;
    for (index, projectile) in game.projectiles.iter_mut().enumerate() {
        if projectile.position.y <= SCREEN_HEIGHT as f32 - (SCREEN_HEIGHT as f32 * 2.0) {
            game.fire_next_projectile = true;
            index_to_delete = index;
            should_delete = true;
        }
        projectile.position.y -= projectile.force.y;
    }
    if should_delete {
        game.projectiles.remove(index_to_delete);
    }
    if rl.is_key_down(KeyboardKey::KEY_W) {
        game.player.position.y -= PLAYER_MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        game.player.position.y += PLAYER_MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        game.player.position.x -= PLAYER_MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        game.player.position.x += PLAYER_MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_SPACE) && game.fire_next_projectile {
        let projectile = Projectile {
            position: Vector2 {
                x: game.player.position.x + (game.player.size.x / 2.0),
                y: game.player.position.y - 20.0
            },
            force: Vector2 { x: 0.0, y: 30.0 },
            color: Color::BLUE,
            radius: 20.0
        };
        game.fire_next_projectile = false;
        game.projectiles.push(projectile);
    }
}

fn draw_game(rl: &mut RaylibHandle, thread: &RaylibThread, game: &mut Game) {
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::PURPLE);
    draw.draw_text("Hello Crusty", 350, 300, 20, Color::WHITE);
    draw.draw_rectangle_v(game.player.position, game.player.size, game.player.color);
    for projectile in game.projectiles.iter_mut() { 
        draw.draw_circle_v(projectile.position, projectile.radius, projectile.color);
    }
}
