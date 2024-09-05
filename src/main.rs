use raylib::{color::Color, ffi::KeyboardKey, math::Rectangle, prelude::RaylibDraw, RaylibHandle};

fn main() {
    // this bad boy here is our 'player'
    let mut rect = Rectangle::new(400.0, 400.0, 100.0, 50.0);

    // initialize raylib and create a window
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Crusty")
        .build();

    // limit the framerate to 250 fps because else inputs will go bonkers
    rl.set_target_fps(250);
    while !rl.window_should_close() {
        // handle keyboard input for our player
        handle_input_rectangle(&mut rl, &mut rect);

        let mut draw = rl.begin_drawing(&thread);
        draw.draw_fps(700, 500);
        draw.clear_background(Color::PURPLE);
        draw.draw_text("Hello Crusty", 350, 300, 20, Color::WHITE);
        //draw our player
        draw.draw_rectangle_rec(rect, Color::RED);
    }
}

/// handle keyboard input for our player and update rectangle position accordingly
fn handle_input_rectangle(rl: &mut RaylibHandle, rect: &mut Rectangle) {
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        rect.y -= 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        rect.y += 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        rect.x -= 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        rect.x += 1.0;
    }
}
