
use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
    },
    event::*,
    keyboard::{ KeyCode, PhysicalKey },
    scene::{ Scene, node::Node },
    script::{ ScriptContext, ScriptTrait },
};
use std::fmt::{ Display, Debug };

pub mod health;


#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "bb240c15-d2dd-4e24-a832-e0af513e4fcf")]
#[visit(optional)]
pub struct Player {
    
    #[visit(optional)]
    #[reflect(hidden)]
    /// Player health and capacity.
    pub health:                     PlayerHealth,

    // ? Player movement activated by controls.
    pub movement_forward:           bool,
    pub movement_backward:          bool,
    pub movement_left:              bool,
    pub movement_right:             bool,

    // ? Block player movement if objects are in the pathway.
    pub movement_forward_block:     bool,
    pub movement_backward_block:    bool,
    pub movement_left_block:        bool,
    pub movement_right_block:       bool,

    // ? Interact with the thing in front of the player.
    pub do_interact:                bool,

    // ? Crouch to sneak or for scene accessibility.
    pub do_crouch:                  bool,

    // ? Open player inventory.
    pub do_inventoryopen:           bool,

    // ? Player camera perspective.
    pub perspective:                PlayerPerspective,
    pub camera:                     Handle<Node>

}

impl Player {

    pub fn new() -> Self {
        Player::default()
    }

    pub fn camera(&self) -> &Handle<Node> {
        &self.camera
    }

    pub fn set_camera(&mut self, new: Handle<Node>) {
        self.camera = new;
    }

}

impl ScriptTrait for Player {
    

    fn on_update(&mut self, context: &mut ScriptContext) {
 
    }

}

#[test]
fn test_playerhealth_safety() {

    let mut playerhealth = PlayerHealth::default();

    // LP and CAP start at 50.
    assert_eq!(playerhealth.lifepoints, 50);
    assert_eq!(playerhealth.capacity, 50);

    // You can't add more LP than your CAP.
    assert_eq!(playerhealth.lifepoints_add(10), &50);

    // Capacity is modified through `.capacity_change()`
    assert_eq!(playerhealth.capacity_change(72), Ok(72));

    // LP does not change with CAP changes; Never overflows.
    assert_eq!(playerhealth.lifepoints, 50);
    assert_eq!(playerhealth.lifepoints_add(u16::MAX), &72);

    // Player is alive as long as LP > 0.
    assert!(playerhealth.is_alive());

    // Subtracting never overflows either.
    assert_eq!(playerhealth.lifepoints_sub(72), &0);
    assert_eq!(playerhealth.lifepoints_sub(1), &0);

    // Zero LP equates death.
    assert!(playerhealth.is_dead());

}

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "18278f64-52a5-44d2-bfb7-b7ecdb0a9924")]
#[visit(optional)]
pub struct PlayerHealth {

    /// Current health points.
    pub lifepoints: u16,

    /// Capacity of health points, as in, the maximum amount of points.
    capacity:   u16,

}

impl PlayerHealth {
    
    pub fn new() -> Self {
        Self::default()
    }

    /// Return health capacity.
    pub fn capacity(&self) -> &u16 {
        &self.capacity
    }

    /// Return lifepoints at the current frame.
    pub fn lifepoints(&self) -> &u16 {
        &self.lifepoints
    }

    /// Change health capacity to the `new` value provided.
    /// Returns a `Result` which indicates if the provided `new` integer is valid, or in other words, anything besides 
    /// zero. A value of zero is illogical and this function returns an `Err` if zero is provided.
    /// 
    /// ```lifepoints = new```
    pub fn capacity_change(&mut self, new: u16) -> Result<u16, ()> {
        if new >= 1 {
            self.capacity = new;
            Ok(self.capacity)
        }
        else {
            Err(())
        }
    }

    /// Add the amount of lifepoints in `add`. Returns the new value.
    /// This function saturates at the bounds of the integer instead of overflowing.
    /// 
    /// ```lifepoints = lifepoints + add ```
    pub fn lifepoints_add(&mut self, add: u16) -> &u16 {
        self.lifepoints = self.lifepoints.saturating_add(add);
        if self.capacity < self.lifepoints {
            self.lifepoints = self.capacity;
        }
        &self.lifepoints
    }

    /// Subtract the amount of lifepoints in `subtract`. Returns the new value.
    /// This function saturates at the bounds of the integer instead of overflowing.
    /// 
    /// ```lifepoints = lifepoints - subtract```
    pub fn lifepoints_sub(&mut self, subtract: u16) -> &u16 {
        self.lifepoints = self.lifepoints.saturating_sub(subtract);
        &self.lifepoints
    }

    pub fn is_alive(&self) -> bool {
        if self.lifepoints != 0 { true }
        else { false }
    }

    pub fn is_dead(&self) -> bool {
        if self.lifepoints == 0 { true }
        else { false }
    }

}

impl Default for PlayerHealth {
    fn default() -> Self {
        PlayerHealth {
            lifepoints:     50,
            capacity:       50
        }
    }
}

impl Display for PlayerHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.lifepoints.to_string(), self.capacity.to_string()))
    }
}

#[derive(Visit, Reflect, Debug, Clone)]
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

    pub fn fov(&self) -> Option<&f32> {
        match self {
            PlayerPerspective::FirstPerson { fov, .. } => Some(fov),
            PlayerPerspective::ThirdPerson { .. } => None
        }
    }

    pub fn pitch(&self) -> &f32 {
        match self {
            PlayerPerspective::FirstPerson { visualfield_pitch, .. } => visualfield_pitch,
            PlayerPerspective::ThirdPerson { visualfield_pitch, .. } => visualfield_pitch
        }
    }

    pub fn yaw(&self) -> &f32 {
        match self {
            PlayerPerspective::FirstPerson { visualfield_yaw, .. } => visualfield_yaw,
            PlayerPerspective::ThirdPerson { visualfield_yaw, .. } => visualfield_yaw
        }
    }

}

impl Display for PlayerPerspective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{perspective}, pitch {pitch}, yaw {yaw}",
            perspective = self.type_name(),
            pitch = self.pitch(),
            yaw = self.yaw()
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
