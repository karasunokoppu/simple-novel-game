use bevy::{ecs::query, prelude::*, state::commands};

use crate::despawn_screen;

use super::DrawUIState;


pub fn text_ui_plugin(app: &mut App) {
    app
    .add_systems(OnEnter(DrawUIState::Text), setup_text_ui)
    .add_systems(OnExit(DrawUIState::Text), despawn_screen::<OnTextUI>);
}

#[derive(Component)]
struct OnTextUI;

#[derive(Component)]
struct TextData {
    name: String,
    text: String,
}

fn setup_text_ui(
    mut commands: Commands,
    text_data: Query<&TextData>
) {
    commands.spawn((//画面全体
        Node{
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            ..default()
        },
        OnTextUI
    )).with_children(|parent|{
        parent.spawn(//テキストボックス
            Node {
                ..default()
            }
        ).with_children(|parent|{
            parent.spawn((//名前
                Node {
                    ..default()
                },
            ));
            parent.spawn(//セリフ
                Node {
                    ..default()
                }
            );
        });
    });
}