
use std::fmt::Display;
use fyrox:: {
    core:: { visitor::prelude::*, reflect::prelude::*, type_traits::prelude::* },
    
};



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
