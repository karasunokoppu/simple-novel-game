use bevy::prelude::*;

use crate::{
    despawn_screen,
    game_states::in_game::{
        draw::text, draw::select, DrawUIState, ImageAssets, NovelGameStates, StoryImageList, StoryWallPaperList,
        WallpaperAssets,
    },
};

pub fn draw_image_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(DrawUIState::Text),
        setup_draw_image.before(text::setup_text_ui),
    )
    .add_systems(
        OnEnter(DrawUIState::Select),
        setup_draw_image.before(select::setup_select_ui),
    )
    .add_systems(OnExit(DrawUIState::Text), despawn_screen::<OnDrawImage>)
    .add_systems(OnExit(DrawUIState::Select), despawn_screen::<OnDrawImage>);
}

#[derive(Component)]
struct OnDrawImage;

pub fn setup_draw_image(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    story_image_list: Res<StoryImageList>,
    novel_game_states: Res<NovelGameStates>,
    wallpaper_assets: Res<WallpaperAssets>,
    story_wallpaper_list: Res<StoryWallPaperList>,
) {
    let mut center_node = (
        ImageNode::new(image_assets.images.get(&2).unwrap().clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Node { ..default() },
    );
    let mut right_node = (
        ImageNode::new(image_assets.images.get(&2).unwrap().clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Node { ..default() },
    );
    let mut left_node = (
        ImageNode::new(image_assets.images.get(&2).unwrap().clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Node { ..default() },
    );
    let mut wallpaper_node = (
        ImageNode::new(image_assets.images.get(&2).unwrap().clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            overflow: Overflow {
                x: OverflowAxis::Visible,
                y: OverflowAxis::Visible,
            },
            ..default()
        },
    );

    for (story, (image_datas, display_images)) in story_image_list.story_data_list.iter() {
        if *story == novel_game_states.story {
            for display_image in display_images.iter() {
                for image_data in image_datas.iter() {
                    if display_image.story_scene_id == novel_game_states.current_story_id as u32 {
                        //left chara image
                        if display_image.left_image_scene_id == image_data.image_id {
                            left_node = (
                                ImageNode::new(
                                    image_assets
                                        .images
                                        .get(&display_image.left_image_scene_id)
                                        .unwrap()
                                        .clone(),
                                ),
                                Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(
                                    image_data.scale.0,
                                    image_data.scale.1,
                                    1.0,
                                )),
                                Node {
                                    width: Val::Percent(100.0 / 3.0),
                                    top: Val::Px(100.0 + display_image.left_top_length),
                                    ..default()
                                },
                            )
                        }
                        //center chara image
                        if display_image.center_image_scene_id == image_data.image_id {
                            center_node = (
                                ImageNode::new(
                                    image_assets
                                        .images
                                        .get(&display_image.center_image_scene_id)
                                        .unwrap()
                                        .clone(),
                                ),
                                Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(
                                    image_data.scale.0,
                                    image_data.scale.1,
                                    1.0,
                                )),
                                Node {
                                    width: Val::Percent(100.0 / 3.0),
                                    top: Val::Px(100.0 + display_image.center_top_length),
                                    ..default()
                                },
                            )
                        }
                        //right chara image
                        if display_image.right_image_scene_id == image_data.image_id {
                            right_node = (
                                ImageNode::new(
                                    image_assets
                                        .images
                                        .get(&display_image.right_image_scene_id)
                                        .unwrap()
                                        .clone(),
                                ),
                                Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(
                                    image_data.scale.0,
                                    image_data.scale.1,
                                    1.0,
                                )),
                                Node {
                                    width: Val::Percent(100.0 / 3.0),
                                    top: Val::Px(100.0 + display_image.right_top_length),
                                    ..default()
                                },
                            )
                        }
                    }
                }
            }
        }
    }
    for (story, (wallpaper_datas, wallpapers)) in story_wallpaper_list.story_data_list.iter() {
        if *story == novel_game_states.story {
            for wallpaper_data in wallpaper_datas.iter() {
                for wallpaper in wallpapers.iter() {
                    if wallpaper_data.story_id == novel_game_states.current_story_id as u32
                        && wallpaper.image_id == wallpaper_data.image_id
                    {
                        wallpaper_node = (
                            ImageNode::new(
                                wallpaper_assets
                                    .images
                                    .get(&wallpaper.image_id)
                                    .unwrap()
                                    .clone(),
                            ),
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceAround,
                                overflow: Overflow {
                                    x: OverflowAxis::Visible,
                                    y: OverflowAxis::Visible,
                                },
                                ..default()
                            },
                        );
                    }
                }
            }
        }
    }
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            OnDrawImage,
        ))
        .with_children(|parent| {
            //背景画像
            parent.spawn(wallpaper_node).with_children(|parent| {
                //キャラ画像
                parent.spawn(left_node);
                parent.spawn(center_node);
                parent.spawn(right_node);
            });
        });
        println!("setup_draw_image");
}
