use bevy::prelude::*;

use crate::{
    despawn_screen,
    game_states::in_game::{
        InGameState,
        NovelGameStates,
        StoryDataList,
        SceneType,
        DrawUIState
    },
};

pub fn conrol_scene_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(InGameState::Control),change_drawui_state)
        .add_systems(OnExit(InGameState::Control), despawn_screen::<OnControl>);
}

// Tag component used to tag entities added on the control scene
#[derive(Component)]
struct OnControl;

fn change_drawui_state(
    data_list: Res<StoryDataList>,
    novel_game_states: Res<NovelGameStates>,
    mut next_draw_ui_state: ResMut<NextState<DrawUIState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for (story, datas) in data_list.story_data_list.iter() {
        for story_scene_data in datas.iter(){
            if story_scene_data.current_id == novel_game_states.next_story_id as u32 {
                println!("{:?}",story_scene_data.scene_type);
                match &story_scene_data.scene_type {
                    SceneType::Text(text) => {
                        next_draw_ui_state.set(DrawUIState::Text);
                        in_game_state.set(InGameState::Draw);
                    }
                    SceneType::Selector(selector) => {
                        next_draw_ui_state.set(DrawUIState::Select);
                        in_game_state.set(InGameState::Draw);
                    }
                }
            }
        }
    }
}