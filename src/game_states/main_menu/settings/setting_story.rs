use bevy::{
    color::palettes::css::CRIMSON, prelude::*, render::render_resource::encase::private::Length,
};
use std::fs;

use crate::{
    game_states::main_menu::settings::{MenuButtonAction, SelectedOption},
    SelectedStory, TEXT_COLOR,
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

// Tag component used to tag entities added on the story settings menu screen
#[derive(Component)]
pub struct OnStorySettingsMenuScreen;

#[derive(Resource, Debug, Default)]
pub struct SaveDatas(Vec<u32>);

pub fn story_settings_menu_setup(
    mut commands: Commands,
    save_story: Res<SelectedStory>,
    save_data: Res<SaveDatas>,
) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands.spawn((
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1100.0),
        OnStorySettingsMenuScreen,
    )).with_children(|parent|{
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                display: Display::Flex,
                                flex_wrap: FlexWrap::Wrap,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(CRIMSON.into()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Volume"), button_text_style.clone()));

                            if save_data.0.length() > 0 {
                                for save_data_iter in 1..(*save_data.0.last().unwrap() + 1) {
                                    //TODO 2.[スクロールに変更する]
                                    let mut entity = parent.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(30.0),
                                            height: Val::Px(65.0),
                                            ..button_node.clone()
                                        },
                                        BackgroundColor(NORMAL_BUTTON),
                                        SelectedStory(save_data_iter),
                                        MenuButtonAction::LoadData(save_data_iter),
                                    ));
                                    entity.insert((
                                        Text::new(format!("{}", save_data_iter)),
                                        button_text_style.clone(),
                                    ));
                                    if *save_story == SelectedStory(save_data_iter) {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            }
                        });
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Button,
                                    button_node.clone(),
                                    BackgroundColor(NORMAL_BUTTON),
                                    MenuButtonAction::RestartPlay,
                                ))
                                .with_child((Text::new("Play"), button_text_style.clone()));
                            parent
                                .spawn((
                                    Button,
                                    button_node.clone(),
                                    BackgroundColor(NORMAL_BUTTON),
                                    MenuButtonAction::BackToMainMenu,
                                ))
                                .with_child((Text::new("Back"), button_text_style.clone()));
                        });
                });
        });
    });
        println!("test");
}

//saves内の処理
pub fn get_save_files_names(mut save_datas: ResMut<SaveDatas>) {
    let entries = fs::read_dir("saves").unwrap();
    let mut file_names = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "ron" {
                    if let Some(file_name) = path.file_stem().and_then(|name| name.to_str()) {
                        file_names.push(file_name.parse().unwrap());
                    }
                }
            }
        }
    }
    file_names.sort_by_key(|s| *s);

    save_datas.0 = file_names;
}
