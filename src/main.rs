use macroquad::prelude::*;
use std::collections::HashSet;

// flappy_bird.rs
mod flappy_bird;
use flappy_bird::flappy_bird::*;

// asteroids.rs
mod asteroids;
use asteroids::asteroids::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui, Skin
};

pub enum GameState {
    Menu,
    Asteroids,
    FlappyBird
}

// Window config for macroquad
fn window_conf() -> Conf {
    Conf {
      window_title: "LP Game Emulator".to_owned(),
      window_width: 1000,
      window_height: 600,
      high_dpi: true,
      window_resizable: false,
      ..Default::default()
    }
}

fn draw_loading_screen(x_midscreen: f32, y_midscreen: f32) {
    let text = "Carregando ...";
    let font_size = 30.;
    let text_size = measure_text(text, None, font_size as _, 1.0);
    draw_text(
        text,
        x_midscreen - text_size.width / 2.,
        y_midscreen - text_size.height / 2.,
        font_size,
        BLACK,
    );
}

const LABEL_SIZE: u16 = 50;

#[macroquad::main(window_conf)]
async fn main() {

    let mut game_state = GameState::Menu;

    let x_midscreen = screen_width() / 2.;
    let y_midscreen = screen_height() / 2.;

    let skin = {
        let button_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(180, 180, 100, 255))
        .font_size(LABEL_SIZE)
        .build();

        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };

    let skin_schema = skin.clone();

    root_ui().push_skin(&skin_schema);

    loop {
        
        clear_background(WHITE);

        match game_state {

            // Estado Menu
            GameState::Menu => {
                let text = "< Simulador de jogos - Linguagens de Programação 2023 >";
                let text_size = measure_text(text, None, TEXT_SIZE, 1.0);
                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 4. - text_size.height / 2.,
                    TEXT_SIZE as f32,
                    TEXT_COLOR,
                );

            }

            // Estado de jogo Asteroids
            GameState::Asteroids => {
                // Tela de carregamento
                clear_background(WHITE);
                draw_loading_screen(x_midscreen, y_midscreen);

                next_frame().await;

                // Chamada do jogo Asteroids
                loop {
                    if !asteroids_game().await {
                        break;
                    }
                }

                // Volta ao estado de Menu
                game_state = GameState::Menu;
            }
            
            // Estado de jogo Flappy Bird
            GameState::FlappyBird => {
                // Tela de carregamento
                clear_background(WHITE);
                draw_loading_screen(x_midscreen, y_midscreen);

                next_frame().await;

                // Chamada do jogo Asteroids
                loop {
                    if !flappy_bird_game().await {
                        break;
                    }
                }

                // Volta ao estado de Menu
                game_state = GameState::Menu;
            }
            
        }

        let mut button_label = "Asteroids";
        let mut label_size = measure_text(button_label, None, LABEL_SIZE, 1.0);

        if root_ui().button(Vec2::new(x_midscreen - label_size.width / 2., y_midscreen), button_label) {
            println!("Play asteroids");
            game_state = GameState::Asteroids;
        }

        button_label = "Flappy Bird";
        label_size = measure_text(button_label, None, LABEL_SIZE, 1.0);

        if root_ui().button(Vec2::new(x_midscreen - label_size.width / 2., y_midscreen + (y_midscreen / 4.)), button_label) {
            println!("Play Flappy Bird");
            game_state = GameState::FlappyBird;
        }

        next_frame().await;
    }

}