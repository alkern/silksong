use bevy::color::Srgba;
use bevy::color::palettes::css::{
    BLUE_VIOLET, CORNFLOWER_BLUE, DARK_ORCHID, INDIGO, REBECCA_PURPLE,
};
use bevy::log::warn;
use bevy::prelude::Vec2;

#[derive(Debug, Default, PartialEq)]
pub enum ColorPalette {
    #[default]
    BlueViolet,
    CornflowerBlue,
    DarkOrchid,
    Indigo,
    RebeccaPurple,
}

impl ColorPalette {
    pub fn to_rgba(&self) -> Srgba {
        match self {
            ColorPalette::BlueViolet => BLUE_VIOLET,
            ColorPalette::CornflowerBlue => CORNFLOWER_BLUE,
            ColorPalette::DarkOrchid => DARK_ORCHID,
            ColorPalette::Indigo => INDIGO,
            ColorPalette::RebeccaPurple => REBECCA_PURPLE,
        }
    }

    pub fn get_random(position: Vec2) -> ColorPalette {
        let position = position * 1000.0;
        let hash = (position.length() % 4.) as u8 + 1;

        match hash {
            1 => ColorPalette::CornflowerBlue,
            2 => ColorPalette::DarkOrchid,
            3 => ColorPalette::Indigo,
            4 => ColorPalette::RebeccaPurple,
            a => {
                warn!("unexpected hash {}", a);
                ColorPalette::BlueViolet
            }
        }
    }
}
