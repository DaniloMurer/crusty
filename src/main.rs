use raylib::{ffi::{KeyboardKey, Vector2}, prelude::RaylibDraw, RaylibHandle, RaylibThread};
use raylib::color::Color;

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
        .size(800, 600)
        .title("Crusty")
        .build();
    rl.set_target_fps(60);
    let mut game = Game::default();
    while !rl.window_should_close() {
        update_game(&mut rl, &mut game);
        draw_game(&mut rl, &thread, &mut game);
    }
}

fn update_game(rl: &mut RaylibHandle, game: &mut Game) {
    //let mut index = 0;
    for projectile in game.projectiles.iter_mut() {
        dbg!(projectile.position.y);
        dbg!(rl.get_screen_height() as f32);
        if projectile.position.y <= rl.get_screen_height() as f32 - (rl.get_screen_height() as f32 * 2.0) {
            game.fire_next_projectile = true;
            //index = game.projectiles.len().clone();
            //index = game.projectiles.iter().position(|&item| item.position.y == projectile.position.y).expect("error while getting index");
            //game.projectiles.remove(index);
        }
        projectile.position.y -= projectile.force.y;
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        game.player.position.y -= 4.0;
    } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        game.player.position.y += 4.0;
    } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        game.player.position.x -= 4.0;
    } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        game.player.position.x += 4.0;
    } else if rl.is_key_down(KeyboardKey::KEY_SPACE) {
        let projectile = Projectile {
            position: Vector2 {
                x: game.player.position.x + (game.player.size.x / 2.0),
                y: game.player.position.y - 20.0
            },
            force: Vector2 { x: 0.0, y: 30.0 },
            color: Color::BLUE,
            radius: 20.0
        };
        if game.fire_next_projectile {
            game.fire_next_projectile = false;
            game.projectiles.push(projectile);
        }
    }
}

fn draw_game(rl: &mut RaylibHandle, thread: &RaylibThread, game: &mut Game) {

    let mut draw = rl.begin_drawing(thread);
    draw.draw_fps(700, 500);
    draw.clear_background(Color::PURPLE);
    draw.draw_text("Hello Crusty", 350, 300, 20, Color::WHITE);
    //draw our player
    draw.draw_rectangle_v(game.player.position, game.player.size, game.player.color);
    for projectile in game.projectiles.iter_mut() { 
        draw.draw_circle_v(projectile.position, projectile.radius, projectile.color);
    }
    /*if game.projectile.should_render {
        draw.draw_circle_v(game.projectile.position, game.projectile.radius, game.projectile.color);
    }*/
}
