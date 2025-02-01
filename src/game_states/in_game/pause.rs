use bevy::prelude::*;

use crate::despawn_screen;

pub fn pause_scene_plugin(app: &mut App) {
    app.init_state::<PauseState>()
        .add_systems(OnEnter(PauseState::Pause), setup_pause_ui)
        // .add_systems(OnEnter(DrawUIState::Select), setup_pause_ui)
        .add_systems(Update, pause_button_system.run_if(in_state(PauseState::Pause)))
        .add_systems(OnExit(PauseState::Pause), despawn_screen::<OnPause>);
}
//TODO 1.[pause時にセーブしたらsavesディレクトリに[数字].ronファイルを生成するようにする]

// Tag component used to tag entities added on the new_game_loading scene
#[derive(Component)]
struct OnPause;

#[derive(Component)]
pub struct PauseButton;

//
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseState {
    Pause,
    #[default]
    Disabled,
}

//Color
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
const UI_BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const UI_BACKGROUND_COLOR: Color = Color::Srgba(Srgba::new(0.2, 0.2, 0.2, 0.8));

fn setup_pause_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1000.0),
            OnPause,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            )).with_children(|parent| {
                parent.spawn((
                    //セリフ
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
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
                    Button,PauseButton
                ));
            });
        });
    println!("Pause ui is spawned");
}

fn pause_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, (With<Button>, With<PauseButton>)),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => *background_color = PRESSED_BUTTON.into(),
            Interaction::Hovered => *background_color = HOVERED_BUTTON.into(),
            Interaction::None => *background_color = UI_BACKGROUND_COLOR.into(),
        }
    }
}

