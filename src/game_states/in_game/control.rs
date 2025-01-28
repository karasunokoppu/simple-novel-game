use bevy::prelude::*;

use crate::{
    despawn_screen,
    game_states::in_game::{DrawUIState, InGameState, NovelGameStates, SceneType, StoryDataList},
    GameState,
};

pub fn conrol_scene_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::Control), change_drawui_state)
        .add_systems(OnExit(InGameState::Control), despawn_screen::<OnControl>);
}

// Tag component used to tag entities added on the control scene
#[derive(Component)]
struct OnControl;

fn change_drawui_state(
    data_list: Res<StoryDataList>,
    mut novel_game_states: ResMut<NovelGameStates>,
    mut next_draw_ui_state: ResMut<NextState<DrawUIState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (_, datas) in data_list.story_data_list.iter() {
        for story_scene_data in datas.iter() {
            if story_scene_data.current_id == novel_game_states.next_story_id as u32 {
                println!("================================");
                println!("> Next SceneType is {:?}.", story_scene_data.scene_type);
                println!("> Next id is {:?}.", novel_game_states.next_story_id);
                println!("================================");

                match &story_scene_data.scene_type {
                    SceneType::Text(_) => {
                        in_game_state.set(InGameState::Draw);
                        next_draw_ui_state.set(DrawUIState::Text);
                        //現在のIDを更新
                        novel_game_states.current_story_id = novel_game_states.next_story_id;
                        println!("> InGameState Control -> Draw");
                        println!("> DrawUIState Disabled -> Text");
                    }
                    SceneType::Selector(_) => {
                        in_game_state.set(InGameState::Draw);
                        next_draw_ui_state.set(DrawUIState::Select);
                        //現在のIDを更新
                        novel_game_states.current_story_id = novel_game_states.next_story_id;
                        println!("> InGameState Control -> Draw");
                        println!("> DrawUIState Disabled -> Select");
                    }
                    SceneType::Finish(finish) => {
                        if finish.text == "finish" {//終了処理
                            in_game_state.set(InGameState::Disabled);
                            next_draw_ui_state.set(DrawUIState::Disabled);
                            game_state.set(GameState::MainMenu);

                            novel_game_states.current_story_id = 1;
                            novel_game_states.next_story_id = 1;
                            //現在のIDを更新
                            novel_game_states.current_story_id = novel_game_states.next_story_id;
                            println!("> InGameState [] -> Disabled");
                            println!("> DrawUIState Disabled -> Disabled");
                            println!("> GameState NewGame -> MainMenu");
                        }else {//次のストーリーへ遷移
                            novel_game_states.story = finish.text.clone();
                            novel_game_states.current_story_id = 1;
                            novel_game_states.next_story_id = 1;

                            in_game_state.set(InGameState::ContinueGameLoading);
                            next_draw_ui_state.set(DrawUIState::Disabled);
                            println!("> InGameState [] -> ContinueGameLoading");
                            println!("> DrawUIState Disabled -> Disabled");
                        }
                    }
                }
            }
        }
    }
}
