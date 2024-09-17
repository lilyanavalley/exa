
use std::fmt::Display;
use fyrox:: {
    core:: { visitor::prelude::*, reflect::prelude::*, type_traits::prelude::* },
    
};
use strum_macros::*;


#[test]
fn test_playerperspective_pitchyaw() {

    // Modify pitch & yaw after creating a new `PlayerPerspective`.
    let mut perspective = PlayerPerspective::default();
    perspective.set_pitch(180.0);
    perspective.set_yaw(90.0);

    println!("{}", perspective);

    // Did setting the new values take effect?
    assert_eq!(perspective.pitch(), 180.0);
    assert_eq!(perspective.yaw(), 90.0);

}

#[test]
fn test_playerperspective_defaults() {

    // PlayerPerspective defaults to `FirstPerson`.
    let firstperson_pp = PlayerPerspective::default();
    // TODO: Replace with new_thirdperson() functionality.
    let thirdperson_pp = PlayerPerspective::ThirdPerson { visualfield_pitch: 0.0, visualfield_yaw: 0.0 };

    assert_eq!(firstperson_pp.fov(), Some(75.0));
    assert_eq!(firstperson_pp.pitch(), 0.0);
    assert_eq!(firstperson_pp.yaw(), 0.0);


    // TODO: Test for thirdperson defaults below.

}

#[test]
fn test_playerperspective_fov() {

    let mut firstperson = PlayerPerspective::default();

    assert_eq!(firstperson.fov(), Some(75.0));

}

/// *PlayerPerspective* tells the game how to position the camera and holds your `pitch` + `yaw`.
#[derive(Visit, Reflect, Debug, AsRefStr, EnumString, VariantNames, Clone, TypeUuidProvider)]
#[type_uuid(id = "54c8c33a-0f39-4ad4-a8f6-152af9707e90")]
pub enum PlayerPerspective {

    FirstPerson {
        // ? Pitch & Yaw for mouse/controller first-person perspective.
        fov:                        f32,
        visualfield_pitch:          f32,
        visualfield_yaw:            f32,
    },

    ThirdPerson {
        visualfield_pitch:          f32,
        visualfield_yaw:            f32,
    }

}

impl PlayerPerspective {

    /// Calls `Self::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates clone of `self.fov` *f32* if it's available.
    /// 
    /// If an FOV is `Some(...)` then Self is `FirstPerson` mode.
    /// If an FOV is `None` then Self is `ThirdPerson` mode.
    pub fn fov(&self) -> Option<f32> {
        match self {
            PlayerPerspective::FirstPerson { fov, .. } => Some(fov.clone()),
            PlayerPerspective::ThirdPerson { .. } => None
        }
    }

    /// Creates clone of `self.visualfield_pitch` *f32*.
    pub fn pitch(&self) -> f32 {
        match self {
            PlayerPerspective::FirstPerson { visualfield_pitch, .. } => visualfield_pitch.clone(),
            PlayerPerspective::ThirdPerson { visualfield_pitch, .. } => visualfield_pitch.clone()
        }
    }

    /// Creates clone of `self.visualfield_yaw` *f32*.
    pub fn yaw(&self) -> f32 {
        match self {
            PlayerPerspective::FirstPerson { visualfield_yaw, .. } => visualfield_yaw.clone(),
            PlayerPerspective::ThirdPerson { visualfield_yaw, .. } => visualfield_yaw.clone()
        }
    }

    /// Sets pitch of `Self` to the new value given through `new`.
    /// Returns a borrowed, new version of `Self`, if you desire.
    pub fn set_pitch(&mut self, new: f32) -> &Self {
        match self {
            PlayerPerspective::FirstPerson { visualfield_pitch, .. } => *visualfield_pitch = new,
            PlayerPerspective::ThirdPerson { visualfield_pitch, .. } => *visualfield_pitch = new,
        };
        self
    }

    /// Sets yaw of `Self` to the new value given through `new`.
    /// Returns a borrowed, new version of `Self`, if you desire.
    pub fn set_yaw(&mut self, new: f32) -> &Self {
        match self {
            PlayerPerspective::FirstPerson { visualfield_yaw, .. } => *visualfield_yaw = new,
            PlayerPerspective::ThirdPerson { visualfield_yaw, .. } => *visualfield_yaw = new
        };
        self
    }

}

impl Display for PlayerPerspective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{perspective}, pitch {pitch}, yaw {yaw}",
            perspective = self.type_name(),
            pitch = self.pitch().to_radians(),
            yaw = self.yaw().to_radians()
        ))
    }
}

impl Default for PlayerPerspective {
    fn default() -> Self {
        PlayerPerspective::FirstPerson {
            fov: 75.0,
            visualfield_pitch: 0.0,
            visualfield_yaw: 0.0
        }
    }
}

