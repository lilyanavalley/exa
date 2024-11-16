
use ron;
use std::{
    any::Any,
    hash::Hash,
    fmt::Debug,
    marker::Send,
    collections::HashMap,
};
use serde:: { Serialize, Deserialize };
use fyrox::{
    core:: { pool::Handle, reflect::prelude::*, visitor::prelude::* }, event::*, gui::{ screen::Screen, widget::WidgetBuilder, wrap_panel::Line, UiNode, UserInterface }, keyboard::*, plugin::PluginContext
};
// use super::SettingsComponent;


mod tests {

    use super::{ InputSettingsComponent };

    #[test]
    fn test_inputsettingscomponent_defaults_safety() {

        let isc = InputSettingsComponent::default();
        // TODO: Write this test to completion.

    }

}

// * Input Settings Component as a subsystem of `super::Settings`.
#[derive(Debug, Serialize, Deserialize)]
pub struct InputSettingsComponent {

    /// All bindings.
    /// 
    /// Associates an input source (like key press) with an action meaningful to the engine (like movement across the
    /// playing field.)
    pub bindings:           HashMap<BindingSources, BindingActions>,

    /// Desktop input settings not otherwise applicable as a binding.
    pub desktop:            desktop_input::DesktopInput,

    /// Gamepad input settings not otherwise applicable as a binding.
    pub gamepad:            gamepad_input::GamepadInput,

}

impl InputSettingsComponent {

    // Default bindings on a QWERTY keyboard.
    fn default_bindings() -> HashMap<BindingSources, BindingActions> {

        let mut map = HashMap::new();
        map.insert(
            BindingSources::Desktop(desktop_input::DesktopInputSources::Key(PhysicalKey::Code(KeyCode::KeyW))),
            BindingActions::linear(LinearDirectionSetting::forward())
        );
        map.insert(
            BindingSources::Desktop(desktop_input::DesktopInputSources::Key(PhysicalKey::Code(KeyCode::KeyS))),
            BindingActions::linear(LinearDirectionSetting::backward())
        );
        map.insert(
            BindingSources::Desktop(desktop_input::DesktopInputSources::Key(PhysicalKey::Code(KeyCode::KeyA))),
            BindingActions::linear(LinearDirectionSetting::left())
        );
        map.insert(
            BindingSources::Desktop(desktop_input::DesktopInputSources::Key(PhysicalKey::Code(KeyCode::KeyD))),
            BindingActions::linear(LinearDirectionSetting::right())
        );
        map.insert(
            BindingSources::Desktop(desktop_input::DesktopInputSources::Key(PhysicalKey::Code(KeyCode::KeyE))),
            BindingActions::interact()
        );

        map

    }

}

impl Default for InputSettingsComponent {

    fn default() -> Self {
        InputSettingsComponent {
            bindings:       Self::default_bindings(),
            desktop:        desktop_input::DesktopInput::default(),
            gamepad:        gamepad_input::GamepadInput::default()
        }
    }

}

// /// A collection of binds and their associated actions.
// /// 
// /// Also contains a *debounce buffer* to filter out repeated messages (like keypress repeat.)
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Bindings<I>
//     where I: Hash + Eq + BindingLayer {

//     pub bindings:   HashMap<I, BindingActions>,
//     pub debounce:   ()

// }

// impl <I> Default for Bindings<I>
//     where I: Hash + Eq + BindingLayer {
//     fn default() -> Self {
//         Self {
//             bindings: HashMap::default(),
//             debounce: ()
//         }
//     }
// }

pub trait BindingLayer<T>
    where T: Hash {

    // Find actions registered to the given input `source`.
    // TODO: Document.
    fn binding_action(&self, source: BindingSources) -> Option<&BindingActions>;
    // Find input sources registered to the given `action`.
    // TODO: Document.
    fn binding_sources(&self, action: BindingActions) -> Option<&[BindingSources]>;

    // Bind an input source to its action.
    // TODO: Document.
    fn bind(&mut self, source: BindingSources, action: BindingActions);

    // Unbind an input source from its action.
    // TODO: Document.
    fn unbind(&mut self, source: BindingSources);

    // Return 'debounce count' for the provided input `source`.
    // Sources of inputs where debounce functionality is necessary will always return `Some()`, while inputs that are
    // not applicable return `None`.
    // TODO: Document better.
    fn debounce(&self, source: BindingSources) -> Option<&u32>;

}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum BindingSources {

    Desktop(desktop_input::DesktopInputSources),

    Gamepad(gamepad_input::GamepadInputSources)

}

/// Game actions associated with input.
#[derive(Debug, Serialize, Deserialize)]
pub enum BindingActions {

    /// Nonlinear movement on joystick actuation.
    MovementNonlinear { x: f32, y: f32, z: f32 },

    /// Linear movement on key/button press.
    MovementLinear (LinearDirectionSetting),

    /// Interact button.
    Interact

}

impl BindingActions {

    pub fn nonlinear(x: f32, y: f32, z: f32) -> Self {
        BindingActions::MovementNonlinear { x, y, z }
    }

    pub fn linear(direction: LinearDirectionSetting) -> Self {
        BindingActions::MovementLinear (direction)
    }

    pub fn interact() -> Self {
        BindingActions::Interact
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub enum LinearDirectionSetting {
    Forward,
    Backward,
    Left,
    Right
}

impl LinearDirectionSetting {

    pub fn forward() -> Self {
        Self::Forward
    }

    pub fn backward() -> Self {
        Self::Backward
    }

    pub fn left() -> Self {
        Self::Left
    }

    pub fn right() -> Self {
        Self::Right
    }

}

pub mod desktop_input {

    use std::{
        any::Any,
        hash::Hash,
        collections::HashMap,
        default::Default
    };
    use serde:: { Serialize, Deserialize };
    use fyrox::{
        event::*,
        core:: { pool::Handle, reflect::prelude::*, visitor::prelude::* },
        dpi::PhysicalPosition,
        keyboard::*
    };


    #[derive(Debug, Serialize, Deserialize)]
    pub struct DesktopInput {

        /// Mouse sensitivity multiplier.
        pub mouse_sensitivity:  f32,

    }

    impl Default for DesktopInput {
        fn default() -> Self {
            DesktopInput {
                mouse_sensitivity:  1.0
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum DesktopInputSources {

        Key(PhysicalKey),

        MouseButton(MouseButton),

        MouseScroll(abstractions::MouseScrollDirection)

    }

    impl From<PhysicalKey> for DesktopInputSources {
        fn from(value: PhysicalKey) -> Self {
            DesktopInputSources::Key(value)
        }
    }

    impl From<MouseButton> for DesktopInputSources {
        fn from(value: MouseButton) -> Self {
            DesktopInputSources::MouseButton(value)
        }
    }

    impl From<abstractions::MouseScrollDirection> for DesktopInputSources {
        fn from(value: abstractions::MouseScrollDirection) -> Self {
            DesktopInputSources::MouseScroll(value)
        }
    }

    mod abstractions {

        use std::ops:: Add;
        use serde:: { Serialize, Deserialize };
        use fyrox::event::MouseScrollDelta;


        /// Direction of mouse wheel scroll.
        /// 
        /// Does not account for horizontal scroll.
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
        pub enum MouseScrollDirection {
            Up,
            Down
        }

        impl From<MouseScrollDelta> for MouseScrollDirection {
            fn from(other: MouseScrollDelta) -> Self {
                match other {
                    MouseScrollDelta::LineDelta(_, v)   => scroller(v),
                    MouseScrollDelta::PixelDelta(pp)    => scroller(pp.y as f32)
                }
            }
        }

        fn scroller(axis: f32) -> MouseScrollDirection {
            if axis.is_sign_positive() { MouseScrollDirection::Up }
            else { MouseScrollDirection::Down }
        }

    }
    
}

pub mod gamepad_input {

    use serde:: { Serialize, Deserialize };
    use gilrs;


    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct GamepadInput {



    }

    pub type GamepadId = gilrs::GamepadId;
    pub type GamepadInputSources = gilrs::ev::Code;

}
