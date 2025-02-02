pub mod control;
pub mod draw;
pub mod loading_game;
pub mod pause;

use bevy::prelude::*;
use pause::PuaseButtonState;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{game_states::in_game::{
    self,
    pause::PauseState,
}, GameState};

pub fn game_plugin(app: &mut App) {
    app.init_state::<InGameState>() //TODO 1.[メインメニューからコンティニューボタンを押して入ったときのInGameStateの初期化方法を変える必要がある]
        .init_state::<DrawUIState>()
        .init_state::<PuaseButtonState>()
        .init_resource::<StoryDataList>()
        .init_resource::<StoryImageList>()
        .init_resource::<StoryWallPaperList>()
        .init_resource::<ImageAssets>()
        .init_resource::<WallpaperAssets>()
        .init_resource::<NovelGameStates>()
        .add_plugins((
            in_game::control::conrol_scene_plugin,
            in_game::draw::draw_scene_plugin,
            in_game::pause::pause_scene_plugin,
            in_game::loading_game::loading_game_plugin,
        ))
        //Pause state
        .add_systems(OnEnter(InGameState::Draw), pause_state_to_pause)
        .add_systems(OnExit(InGameState::Draw), pause_state_to_disabled)
        .add_systems(OnEnter(GameState::InGame), state_to_new_game_loading);
}

fn state_to_new_game_loading(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::LoadingGame);
}

fn pause_state_to_pause(
    mut pause_state: ResMut<NextState<PauseState>>
){
    pause_state.set(PauseState::Pause)
}
fn pause_state_to_disabled(
    mut pause_state: ResMut<NextState<PauseState>>
){
    pause_state.set(PauseState::Disabled)
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    LoadingGame,
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
            next_story_id: 1,
        }
    }
}
#[derive(Resource)]
pub struct ImageAssets {
    images: HashMap<u32, Handle<Image>>,
}
impl Default for ImageAssets {
    fn default() -> Self {
        let mut images = HashMap::new();
        images.insert(1, Handle::default());

        ImageAssets { images: images }
    }
}

#[derive(Resource)]
pub struct WallpaperAssets {
    images: HashMap<u32, Handle<Image>>,
}
impl Default for WallpaperAssets {
    fn default() -> Self {
        let mut images = HashMap::new();
        images.insert(1, Handle::default());

        WallpaperAssets { images: images }
    }
}

//ゲームUIのstate管理
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DrawUIState {
    Text,
    Select,
    #[default]
    Disabled,
}

//ストーリーのテキストをロードして保管するための構造体
#[derive(Resource, Default, Debug)]
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
pub enum SceneType {
    Text(Text),
    Selector(Selector),
    Finish(Finish),
}

#[derive(Deserialize, Debug)]
pub struct Text {
    name: String,
    text: String,
    next_id: u32, //Todo ←これいる？
}

#[derive(Deserialize, Debug)]
pub struct Selector {
    choice01: Choice,
    choice02: Choice,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
struct Choice {
    text: String,
    next_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct Finish {
    text: String,
}

//ストーリーの画像をロードして保管するための構造体
#[derive(Resource, Default)]
pub struct StoryImageList {
    story_data_list: HashMap<String, (Vec<ImageData>, Vec<DisplayImage>)>,
}

#[derive(Component, Deserialize, Debug)]
pub struct ImageData {
    image_id: u32,
    chara: String,
    face: String,
    scale: (f32, f32),
}

#[derive(Component, Deserialize, Debug)]
pub struct DisplayImage {
    story_scene_id: u32,
    //center
    center_image_scene_id: u32,
    center_top_length: f32,
    //right
    right_image_scene_id: u32,
    right_top_length: f32,
    //left
    left_image_scene_id: u32,
    left_top_length: f32,
}

//背景画像をロードして保管するための構造体
#[derive(Resource, Default)]
pub struct StoryWallPaperList {
    pub story_data_list: HashMap<String, (Vec<WallPaperData>, Vec<Wallpapers>)>,
}

#[derive(Component, Deserialize, Debug)]
pub struct WallPaperData {
    story_id: u32,
    image_id: u32,
}

#[derive(Component, Deserialize, Debug)]
pub struct Wallpapers {
    image_id: u32,
    wallpaper_name: String,
}
