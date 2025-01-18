pub mod control;
pub mod draw;
pub mod new_game_loading;
pub mod pause;

use bevy::prelude::*;
use std::collections::HashMap;
use serde::Deserialize;

use crate::{despawn_screen, GameState, game_states::in_game};

pub fn game_plugin(app: &mut App) {
    app//.add_systems(OnEnter(GameState::Game), game_setup)
        //.add_systems(Update, game.run_if(in_state(GameState::Game)))
        .init_state::<InGameState>()
        .init_resource::<in_game::StoryDataList>()
        .add_plugins((
            in_game::control::conrol_scene_plugin,
            in_game::draw::draw_scene_plugin,
            in_game::pause::pause_scene_plugin,
            in_game::new_game_loading::new_game_loading_plugin,
        ))
        .add_systems(OnEnter(GameState::NewGame), (
            state_to_new_game_loading,
        ))
        .add_systems(OnExit(GameState::NewGame), (
            despawn_screen::<OnGameScreen>,
            state_to_disabled,
        ));
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum InGameState {
    NewGameLoading,
    Pause,
    Control,
    Draw,
    #[default]
    Disabled,
}

//ゲーム内の状態管理
#[derive(Resource)]
pub struct NovelGameStates {
    story: String,
    current_story_id: i32,
    next_story_id: i32,
}
impl Default for NovelGameStates {
    fn default() -> NovelGameStates {
        NovelGameStates {
            story: "story01".to_string(),
            current_story_id: 1,
            next_story_id: 2,
        }
    }
}

//ストーリーのテキストをロードして保管するための構造体
#[derive(Resource, Default)]
pub struct StoryDataList {
    story_data_list: HashMap<String, Vec<StorySceneData>>,
}

#[derive(Component, Deserialize, Debug)]
pub struct StorySceneData {
    current_id: u32,
    scene_type: SceneType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum SceneType {
    Text(Text),
    Selector(Selector),
}

#[derive(Deserialize, Debug)]
struct Text {
    name: String,
    text: String,
    next_id: u32,//Todo ←これいる？
}

#[derive(Deserialize, Debug)]
struct Selector {
    choice01: Choice,
    choice02: Choice,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
struct Choice {
    text: String,
    next_id: u32,
}

fn state_to_new_game_loading(
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    in_game_state.set(InGameState::NewGameLoading);
}

fn state_to_disabled(
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    in_game_state.set(InGameState::Disabled);
}