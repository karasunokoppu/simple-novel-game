use bevy::prelude::*;

use crate::{
    despawn_screen,
    game_states::in_game::{
        draw::draw_img, pause::PauseButton, DrawUIState, InGameState, NovelGameStates, SceneType, StoryDataList
    },
    TEXT_COLOR,
};

pub fn text_ui_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(DrawUIState::Text),
        setup_text_ui.after(draw_img::setup_draw_image),
    )
    .add_systems(Update, button_system.run_if(in_state(DrawUIState::Text)))
    .add_systems(OnExit(DrawUIState::Text), despawn_screen::<OnTextUI>);
}

#[derive(Component)]
struct OnTextUI;

//TODO 2.[色のテーマを一元管理するようにする]
//Color
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
const UI_BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const UI_BACKGROUND_COLOR: Color = Color::Srgba(Srgba::new(0.2, 0.2, 0.2, 0.8));

pub fn setup_text_ui(
    mut commands: Commands,
    data_list: Res<StoryDataList>,
    novel_game_states: Res<NovelGameStates>,
) {
    //ストーリーのデータをロード
    let mut name_text = String::new();
    let mut main_text = String::new();

    for (_, datas) in data_list.story_data_list.iter() {
        for story_scene_data in datas.iter() {
            if story_scene_data.current_id == novel_game_states.current_story_id as u32 {
                name_text = match &story_scene_data.scene_type {
                    SceneType::Text(text) => text.name.clone(),
                    _ => "Error".to_string(),
                };
                main_text = match &story_scene_data.scene_type {
                    SceneType::Text(text) => text.text.clone(),
                    _ => "Error".to_string(),
                };
            }
        }
    }
    //GUIのセットアップ
    commands
        .spawn((
            //画面全体
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnTextUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    //テキストボックス
                    Node {
                        width: Val::Vw(80.0),
                        height: Val::Px(250.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::End,
                        ..default()
                    },
                )
                .with_children(|parent| {
                    parent
                        .spawn((
                            //名前
                            Node {
                                width: Val::Px(250.0),
                                height: Val::Px(50.0),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                border: UiRect::new(
                                    Val::Px(3.0),
                                    Val::Px(3.0),
                                    Val::Px(3.0),
                                    Val::ZERO,
                                ),
                                ..default()
                            },
                            BorderColor(UI_BORDER_COLOR),
                            BackgroundColor(UI_BACKGROUND_COLOR),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(name_text),
                                TextFont {
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                            ));
                        });
                    parent
                        .spawn((
                            //セリフ
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(200.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                justify_items: JustifyItems::End,
                                border: UiRect::new(
                                    Val::Px(3.0),
                                    Val::Px(3.0),
                                    Val::Px(3.0),
                                    Val::Px(3.0),
                                ),
                                ..default()
                            },
                            BorderColor(UI_BORDER_COLOR),
                            BackgroundColor(UI_BACKGROUND_COLOR),
                            Button,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(main_text),
                                TextFont {
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
        println!("setup_text_ui");
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, (With<Button>, Without<PauseButton>)),
    >,
    data_list: Res<StoryDataList>,
    mut novel_game_states: ResMut<NovelGameStates>,
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut draw_ui_state: ResMut<NextState<DrawUIState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();

                //next_idを変更
                for (_, datas) in data_list.story_data_list.iter() {
                    for story_scene_data in datas.iter() {
                        if story_scene_data.current_id == novel_game_states.current_story_id as u32
                        {
                            novel_game_states.next_story_id = match &story_scene_data.scene_type {
                                SceneType::Text(text) => text.next_id as i32,
                                _ => {
                                    panic!("Wrong SceneType!");
                                }
                            }
                        }
                    }
                }
                //state変更
                in_game_state.set(InGameState::Control);
                draw_ui_state.set(DrawUIState::Disabled);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON.into(),
            Interaction::None => *background_color = UI_BACKGROUND_COLOR.into(),
        }
    }
}
