mod game_states;

use std::any::type_name;

use crate::game_states::in_game::{
    pause::{PauseButtonState, PauseState},
    DrawUIState, InGameState,
};
use bevy::prelude::*;
use game_states::main_menu::{settings::setting_story::SaveDatas, MenuState, LoadDataEvent};

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
        .init_resource::<SaveDatas>()
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .insert_resource(SelectedStory(1))

        .add_event::<LoadDataEvent>()
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (
                // in_game_state_change_detect,
                // game_state_change_detect,
                // draw_ui_state_change_detect,
                // pause_state_change_detect,
                state_change_detect::<InGameState>,
                state_change_detect::<GameState>,
                state_change_detect::<DrawUIState>,
                state_change_detect::<PauseState>,
                state_change_detect::<PauseButtonState>,
                state_change_detect::<MenuState>,
            ),
        )
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

fn state_change_detect<T: States + Copy>(
    game_states: Res<State<T>>,
    mut last_state: Local<Option<T>>,
) {
    if Some(game_states.get().clone()) != *last_state {
        *last_state = Some(game_states.get().clone());
        let current_state = format!("{:?}", last_state.unwrap());

        let mut t_type = type_name::<T>();
        if let Some(pos) = t_type.rfind("::") {
            t_type = &t_type[pos + 2..];
        }
        println!("{} {} {} {}", ">", t_type, "changed to", current_state,);
    }
}
//TODO 3.[各ボタンを押したときに、ターミナルにログが出るようにする]
