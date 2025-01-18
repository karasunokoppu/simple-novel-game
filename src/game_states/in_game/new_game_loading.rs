use bevy::prelude::*;
use std::fs::File;
use ron::*;

use crate::{
    despawn_screen,
    game_states::in_game::{
        InGameState,
        StoryDataList,
        StorySceneData,
        NovelGameStates,
    }
};

pub fn new_game_loading_plugin(app: &mut App){
    app
    .init_resource::<NovelGameStates>()
    .add_systems(OnEnter(InGameState::NewGameLoading), (
        load_story_new_game,
    ))
    .add_systems(OnExit(InGameState::NewGameLoading), despawn_screen::<OnNewGameLoading>);
}

// Tag component used to tag entities added on the new_game_loading scene
#[derive(Component)]
struct OnNewGameLoading;

fn load_story_new_game(
    mut data_list: ResMut<StoryDataList>,
    novel_game_states: Res<NovelGameStates>,
    mut in_game_state: ResMut<NextState<InGameState>>,
){
    //[story data].ronを開く//
    let path = format!("assets/text_data/{}.ron", novel_game_states.story);
    let file = File::open(path).expect("fail opening file");

    //ストーリーデータのdeserialize
    let story_scene_datas: Vec<StorySceneData> = match ron::de::from_reader(file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load StorySceneData: {}", e);
            std::process::exit(1);
        }
    };
    println!("{}: {:?}", novel_game_states.story, story_scene_datas);

    //Todo (ストーリ名、データリスト)のハッシュマップに入れる必要ある？//
    data_list.story_data_list
        .insert(novel_game_states.story.to_string(), story_scene_datas);

    //state to control
    in_game_state.set(InGameState::Control);
}