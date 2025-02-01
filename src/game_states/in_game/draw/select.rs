use bevy::prelude::*;

use crate::{
    despawn_screen,
    game_states::in_game::{
        draw::draw_img, DrawUIState, InGameState, NovelGameStates, SceneType, StoryDataList,
    },
    TEXT_COLOR,
};

#[derive(Component)]
enum SelectChoice {
    Choice01,
    Choice02,
}

pub fn select_ui_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(DrawUIState::Select),
        setup_select_ui.after(draw_img::setup_draw_image),
    )
    .add_systems(Update, button_system.run_if(in_state(DrawUIState::Select)))
    .add_systems(OnExit(DrawUIState::Select), despawn_screen::<OnSelectUI>);
}

#[derive(Component)]
struct OnSelectUI;

//Color
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
const UI_BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const UI_BACKGROUND_COLOR: Color = Color::Srgba(Srgba::new(0.2, 0.2, 0.2, 0.8));

pub fn setup_select_ui(
    mut commands: Commands,
    data_list: Res<StoryDataList>,
    novel_game_states: Res<NovelGameStates>,
) {
    //ストーリーのデータをロード
    let mut choice01_text = String::new();
    let mut choice02_text = String::new();

    for (_, datas) in data_list.story_data_list.iter() {
        for story_scene_data in datas.iter() {
            if story_scene_data.current_id == novel_game_states.current_story_id as u32 {
                choice01_text = match &story_scene_data.scene_type {
                    SceneType::Selector(selector) => selector.choice01.text.clone(),
                    _ => "Error".to_string(),
                };
                choice02_text = match &story_scene_data.scene_type {
                    SceneType::Selector(selector) => selector.choice02.text.clone(),
                    _ => "Error".to_string(),
                };
            }
        }
    }

    let button_node = Node {
        width: Val::Px(600.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(3.0)),
        ..default()
    };

    commands
        .spawn((
            //画面全体
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            OnSelectUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(80.0),
                    margin: UiRect::vertical(Val::Percent(1.7)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent //choice01
                        .spawn(Node { ..default() })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Button,
                                    SelectChoice::Choice01,
                                    button_node.clone(),
                                    BorderColor(UI_BORDER_COLOR),
                                    BackgroundColor(UI_BACKGROUND_COLOR),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new(choice01_text),
                                        TextFont {
                                            font_size: 30.0,
                                            ..default()
                                        },
                                        TextColor(TEXT_COLOR),
                                    ));
                                });
                        });
                    parent //choice02
                        .spawn(Node { ..default() })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Button,
                                    SelectChoice::Choice02,
                                    button_node.clone(),
                                    BorderColor(UI_BORDER_COLOR),
                                    BackgroundColor(UI_BACKGROUND_COLOR),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new(choice02_text),
                                        TextFont {
                                            font_size: 30.0,
                                            ..default()
                                        },
                                        TextColor(TEXT_COLOR),
                                    ));
                                });
                        });
                });
        });
        println!("setup_select_ui");
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SelectChoice),
        (Changed<Interaction>, With<Button>),
    >,
    data_list: Res<StoryDataList>,
    mut novel_game_states: ResMut<NovelGameStates>,
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut draw_ui_state: ResMut<NextState<DrawUIState>>,
) {
    for (interaction, mut background_color, select_choice) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();

                //next_idを変更
                for (_, datas) in data_list.story_data_list.iter() {
                    for story_scene_data in datas.iter() {
                        if story_scene_data.current_id == novel_game_states.current_story_id as u32
                        {
                            novel_game_states.next_story_id = match &story_scene_data.scene_type {
                                SceneType::Selector(selector) => match select_choice {
                                    SelectChoice::Choice01 => selector.choice01.next_id as i32,
                                    SelectChoice::Choice02 => selector.choice02.next_id as i32,
                                },
                                _ => {
                                    panic!("Wrong SceneType!");
                                }
                            }
                        }
                    }
                }
                //state変更
                draw_ui_state.set(DrawUIState::Disabled);
                in_game_state.set(InGameState::Control);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON.into(),
            Interaction::None => *background_color = UI_BACKGROUND_COLOR.into(),
        }
    }
}
