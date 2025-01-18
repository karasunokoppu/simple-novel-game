pub mod draw_img;
pub mod select;
pub mod text;

use bevy::prelude::*;

use crate::{despawn_screen, game_states::in_game::InGameState};

pub fn draw_scene_plugin(app: &mut App) {
    app
        .add_plugins((
            text::text_ui_plugin,
            select::select_ui_plugin,
        ))
        .add_systems(OnExit(InGameState::Draw), despawn_screen::<OnDraw>);
}

// Tag component used to tag entities added on the new_game_loading scene
#[derive(Component)]
struct OnDraw;