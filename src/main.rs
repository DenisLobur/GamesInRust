use rusty_engine::prelude::*;

mod race;

#[derive(Resource)]
struct GameState {
    current_screen: String,
}

fn main() {
    let mut game = Game::new();

    game.window_settings(Window {
        title: "Choose Game".into(),
        resizable: false,
        decorations: true, // change to false to hide the window decorations
        resolution: WindowResolution::new(800.0, 600.0),
        ..Default::default()
    });

    let main_title = game.add_text("main_screen_title", "Select Game");
    main_title.translation = Vec2::new(0.0, 200.0);

    let race_title = game.add_text("main_screen_race", "Road Race");
    race_title.translation = Vec2::new(0.0, 100.0);

    let shoot_title = game.add_text("main_screen_shoot", "Car Shoot");
    shoot_title.translation = Vec2::new(0.0, 50.0);

    let cannon_title = game.add_text("main_screen_cannon", "Cannon Practice");
    cannon_title.translation = Vec2::new(0.0, 0.0);

    let invader_title = game.add_text("main_screen_invader", "Space Invaders");
    invader_title.translation = Vec2::new(0.0, -50.0);

    game.add_logic(main_menu_logic);
    game.run(GameState {
        current_screen: "main_menu".to_string(),
    });
}

fn main_menu_logic(engine: &mut Engine, state: &mut GameState) {
    if state.current_screen != "main_menu" {
        return;
    }

    let labels = [
        "main_screen_race",
        "main_screen_shoot",
        "main_screen_cannon",
        "main_screen_invader",
    ];

    for &label in &labels {
        hoover_text(engine, label);
    }
}

fn hoover_text(engine: &mut Engine, label: &str) {
    let mouse_position = engine.mouse_state.location();
    let text_pos: Vec2 = {
        let text = engine.texts.get(label).unwrap();
        text.translation
    };
    let mouse_over = (mouse_position.unwrap_or(Vec2::new(0.0, 0.0)).x - text_pos.x).abs() < 100.0
        && (mouse_position.unwrap_or(Vec2::new(0.0, 0.0)).y - text_pos.y).abs() < 30.0;
    let text = engine.texts.get_mut(label).unwrap();
    if mouse_over {
        text.font_size = 40.0;
    } else {
        text.font_size = 30.0;
    }

    // if mouse_over && engine.mouse_state.just_pressed(MouseButton::Left) {
    //
    // }
}
