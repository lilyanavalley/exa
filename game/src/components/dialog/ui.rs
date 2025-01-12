
// This file is part of EXA.
// EXA is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as 
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// EXA is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with EXA. If not, see 
// <https://www.gnu.org/licenses/>.

use fyrox::gui::core::color::Color;
use serde:: { Serialize, Deserialize };


/// State of visibility in dialog UI.
#[derive(Debug, Default, PartialEq)]
pub enum UiVisibility {

    /// Hidden from view.
    #[default]
    Hidden,
    
    /// Transitioning to the hidden state.
    TransitionHidden,

    /// Transitioning to the shown state.
    TransitionShown,

    /// Shown on screen.
    Shown

}

impl UiVisibility {

    /// Alias to `UiVisibility::Hidden`.
    pub fn hidden() -> Self {
        UiVisibility::Hidden
    }

    /// Alias to `UiVisibility::TransitionHidden`.
    pub fn hidden_transition() -> Self {
        UiVisibility::TransitionHidden
    }

    /// Alias to `UiVisibility::TransitionShown`.
    pub fn shown_transition() -> Self {
        UiVisibility::TransitionShown
    }

    /// Alias to `UiVisibility::Shown`.
    pub fn shown() -> Self {
        UiVisibility::Shown
    }

}

/// A Serializable Color for the Dialog UI.
/// 
/// Contains [RGBA channels](https://en.wikipedia.org/wiki/RGBA_color_model) with 8-bits per channel.
/// Implements `From<fyrox_core::color::Color>` and `Into<fyrox_core::color::Color>` to convert into game engine types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SerializedColor {

    r:  u8,
    g:  u8,
    b:  u8,
    a:  u8

}

impl From<Color> for SerializedColor {
    fn from(value: Color) -> Self {
        SerializedColor {
            r:  value.r,
            g:  value.g,
            b:  value.b,
            a:  value.a
        }
    }
}

impl Into<Color> for SerializedColor {
    fn into(self) -> Color {
        Color {
            r:  self.r,
            g:  self.g,
            b:  self.b,
            a:  self.a
        }
    }
}

impl Default for SerializedColor {
    fn default() -> Self {
        SerializedColor {
            r:  128,
            g:  128,
            b:  128,
            a:  255
        }
    }
}

mod tests {

    use super::*;


    #[test]
    fn test_uivisibility() {

        let uiv = UiVisibility::shown();
        assert_eq!(uiv, UiVisibility::Shown);

        let uiv = UiVisibility::hidden();
        assert_eq!(uiv, UiVisibility::Hidden);

        let uiv = UiVisibility::shown_transition();
        assert_eq!(uiv, UiVisibility::TransitionShown);

        let uiv = UiVisibility::hidden_transition();
        assert_eq!(uiv, UiVisibility::TransitionHidden);

    }

    #[test]
    fn test_uivisibility_default() {
        assert_eq!(UiVisibility::default(), UiVisibility::Hidden)
    }

    #[test]
    fn test_serializedcolor_default() {
        assert_eq!(SerializedColor::default(), SerializedColor {
            r: 128,
            g: 128,
            b: 128,
            a: 255
        });
    }

    #[test]
    fn test_serializedcolor_into_sanity() {

        let color = Color { r: 128, g: 128, b: 128, a: 255 };
        let serializedcolor = SerializedColor { r: 128, g: 128, b: 128, a: 255 };

        // Conversion test using .into() to dispatch a type of the equivalent
        assert_eq!(color, serializedcolor.into());

    }

    #[test]
    fn test_serializedcolor_from_sanity() {

        let color = Color { r: 128, g: 128, b: 128, a: 255 };
        let serializedcolor = SerializedColor { r: 128, g: 128, b: 128, a: 255 };

        // Conversion test using .into() to dispatch a type of the equivalent
        assert_eq!(SerializedColor::from(color), serializedcolor);

    }

}

