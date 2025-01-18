use bevy::prelude::*;

use crate::{
    despawn_screen,
    game_states::in_game::{
        InGameState,
        StoryDataList,
        NovelGameStates
    }
};

pub fn conrol_scene_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(InGameState::Control),set_next_id)
        .add_systems(OnExit(InGameState::Control), despawn_screen::<OnControl>);
}

// Tag component used to tag entities added on the control scene
#[derive(Component)]
struct OnControl;

fn set_next_id(
    data_list: Res<StoryDataList>,
    novel_game_states: Res<NovelGameStates>
) {
    for (story, datas) in data_list.story_data_list.iter() {
        for story_scene_data in datas.iter(){
            if story_scene_data.current_id == 1 {
                println!("{:?}",story_scene_data.scene_type);
            }
        }
    }
}