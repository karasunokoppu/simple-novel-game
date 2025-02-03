use bevy::prelude::*;

use crate::{
    game_states::in_game::pause::{
        InPauseButtonAction, PauseButtonMarker, PauseButtonNotPauseMarker, PauseButtonPauseMarker,
    },
    game_states::main_menu::{settings::MenuButtonAction, MenuState},
};

// //! save画面のボタン処理をメインメニューから入ったときと変更しています

//Pauseボタンを押したあとの処理
//Color
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
const UI_BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const UI_BACKGROUND_COLOR: Color = Color::Srgba(Srgba::new(0.2, 0.2, 0.2, 0.8));

//PauseButtonStateがPressedのときに、表示されるウィンドウ
pub fn flip_to_pause_node(
    mut commands: Commands,
    pause_entities: Query<Entity, With<PauseButtonPauseMarker>>,
    pause_button_entities: Query<Entity, With<PauseButtonMarker>>,
) {
    let added_node = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                // grid_template_columns: vec![GridTrack::min_content(), GridTrack::auto()],
                grid_template_rows: vec![
                    GridTrack::min_content(),
                    GridTrack::min_content(),
                    GridTrack::auto(),
                ],
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            PauseButtonNotPauseMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                //Pause, NotPauseの切り替え
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(30.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::new(Val::Px(3.0), Val::Px(3.0), Val::Px(3.0), Val::Px(3.0)),
                    display: Display::Grid,
                    ..default()
                },
                BorderColor(UI_BORDER_COLOR),
                BackgroundColor(UI_BACKGROUND_COLOR),
                Button,
                PauseButtonMarker,
            ));
            parent
                .spawn(Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        //saveボタン //TODO 1.[saveボタン]
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(60.0),
                            padding: UiRect::all(Val::Px(10.0)),
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
                        InPauseButtonAction::Save,
                        PauseButtonNotPauseMarker,
                    ));
                    parent.spawn((
                        //loadボタン //TODO 1.[loadボタン]
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(60.0),
                            padding: UiRect::all(Val::Px(10.0)),
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
                        InPauseButtonAction::Load,
                        PauseButtonNotPauseMarker,
                    ));
                });
            parent.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            });
        })
        .id();

    for pause_entity in pause_button_entities.iter() {
        commands.entity(pause_entity).despawn();
    }

    for pause_entity in pause_entities.iter() {
        commands
            .entity(pause_entity)
            .insert_children(0, &[added_node]);
    }
}

pub fn in_pause_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &InPauseButtonAction),
        (
            Changed<Interaction>,
            (With<Button>, With<PauseButtonNotPauseMarker>),
        ),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, mut background_color, in_pause_button_acrion) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();

                match in_pause_button_acrion {
                    InPauseButtonAction::Save => {
                        menu_state.set(MenuState::SettingsStory); //TODO [変更]
                    }
                    InPauseButtonAction::Load => {
                        menu_state.set(MenuState::SettingsStory);
                    }
                }
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON.into(),
            Interaction::None => *background_color = UI_BACKGROUND_COLOR.into(),
        }
    }
}

pub fn in_pause_in_save_button_system(
    mut menu_interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, mut background_color, menu_button_action) in &mut menu_interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                match *menu_button_action {
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Disabled),
                    MenuButtonAction::RestartPlay => {
                        menu_state.set(MenuState::Disabled) //TODO [変更]
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON.with_alpha(1.0).into(),
            Interaction::None => *background_color = UI_BACKGROUND_COLOR.with_alpha(1.0).into(),
        }
    }
}
