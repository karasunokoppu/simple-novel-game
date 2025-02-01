pub mod draw_img;
pub mod select;
pub mod text;

use bevy::prelude::*;

use crate::{despawn_screen, game_states::in_game::InGameState};

pub fn draw_scene_plugin(app: &mut App) {
    app.add_plugins(draw_img::draw_image_plugin)
        .add_plugins((select::select_ui_plugin, text::text_ui_plugin))
        .add_systems(OnExit(InGameState::Draw), (
            // pause_state_to_disabled,
            despawn_screen::<OnDraw>,
        ));
}

// Tag component used to tag entities added on the new_game_loading scene
#[derive(Component)]

struct OnDraw;