mod game_states;

use bevy::prelude::*;
use colored::Colorize;
use crate::game_states::in_game::{
    InGameState,
    DrawUIState,
    pause::PauseState,
};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    MainMenu,
    InGame,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct SelectedStory(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                //windowの設定
                title: "bevy novel game".to_string(),
                ..default()
            }),
            ..default()
        }))
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .insert_resource(SelectedStory(1))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (
            in_game_state_change_detect,
            game_state_change_detect,
            draw_ui_state_change_detect,
            pause_state_change_detect,
        ))
        // Adds the plugins for each state
        .add_plugins((
            game_states::splash::splash_plugin,
            game_states::main_menu::menu_plugin,
            game_states::in_game::game_plugin,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn in_game_state_change_detect(
    in_game_states: Res<State<InGameState>>,
    mut last_state: Local<Option<InGameState>>
){
    if Some(in_game_states.get().clone()) != *last_state {
        *last_state = Some(in_game_states.get().clone());
        let current_state = format!("{:?}", last_state.unwrap());
        println!("{} {} {} {}",
            ">",
            "InGameState".blue(),
            "changed to",
            current_state.blue(),
        );
    }
}

fn game_state_change_detect(
    game_states: Res<State<GameState>>,
    mut last_state: Local<Option<GameState>>
){
    if Some(game_states.get().clone()) != *last_state {
        *last_state = Some(game_states.get().clone());
        let current_state = format!("{:?}", last_state.unwrap());
        println!("{} {} {} {}",
            ">",
            "GameState".red(),
            "changed to",
            current_state.red(),
        );
    }
}

fn draw_ui_state_change_detect(
    game_states: Res<State<DrawUIState>>,
    mut last_state: Local<Option<DrawUIState>>
){
    if Some(game_states.get().clone()) != *last_state {
        *last_state = Some(game_states.get().clone());
        let current_state = format!("{:?}", last_state.unwrap());
        println!("{} {} {} {}",
            ">",
            "DrawUIState".green(),
            "changed to",
            current_state.green(),
        );
    }
}

fn pause_state_change_detect(
    game_states: Res<State<PauseState>>,
    mut last_state: Local<Option<PauseState>>
){
    if Some(game_states.get().clone()) != *last_state {
        *last_state = Some(game_states.get().clone());
        let current_state = format!("{:?}", last_state.unwrap());
        println!("{} {} {} {}",
            ">",
            "PauseState".bright_yellow(),
            "changed to",
            current_state.bright_yellow(),
        );
    }
}
//TODO 2.[各ボタンを押したときに、ターミナルにログが出るようにする]