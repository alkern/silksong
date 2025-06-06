use crate::core::model::TriggerColor;
use bevy::color::Srgba;
use bevy::color::palettes::css::{
    BLUE_VIOLET, CORNFLOWER_BLUE, DARK_ORCHID, INDIGO, REBECCA_PURPLE,
};
use bevy::log::warn;
use bevy::prelude::{Vec2, Vec4};

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
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

    pub fn enumerate() -> Vec<ColorPalette> {
        vec![
            ColorPalette::BlueViolet,
            ColorPalette::CornflowerBlue,
            ColorPalette::DarkOrchid,
            ColorPalette::Indigo,
            ColorPalette::RebeccaPurple,
        ]
    }
}

impl From<&ColorPalette> for Vec4 {
    fn from(value: &ColorPalette) -> Self {
        let rgba = value.to_rgba();
        Vec4::new(rgba.red, rgba.green, rgba.blue, rgba.alpha)
    }
}

impl From<&TriggerColor> for ColorPalette {
    fn from(value: &TriggerColor) -> Self {
        value.0
    }
}
