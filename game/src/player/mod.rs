
//! 
//! TODO
//! 

use fyrox::{
    asset::manager::ResourceManager, core::{
        algebra::*,
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*
    }, engine, event::*, gui::window, keyboard::{ KeyCode, PhysicalKey }, scene::{ self, camera::CameraBuilder, node::Node, Scene }, script::{ ScriptContext, ScriptMessage, ScriptTrait }
};
// use strum_macros::*;
use tracing::{ trace, info, warn, error, instrument };
use std::fmt::{ Display, Debug };
use crate::utilities::*;

pub mod health;
pub mod skybox;
pub mod perspective;


#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "bb240c15-d2dd-4e24-a832-e0af513e4fcf")]
#[visit(optional)]
pub struct Player {
    
    // ? Player health and capacity.
    // #[visit(optional)]
    // #[reflect(hidden)]
    pub health:                     health::PlayerHealth,

    // ? Player movement activated by controls.
    #[reflect(hidden)]
    #[visit(skip)]
    pub movement_forward:           bool,
    #[reflect(hidden)]
    #[visit(skip)]
    pub movement_backward:          bool,
    #[reflect(hidden)]
    #[visit(skip)]
    pub movement_left:              bool,
    #[reflect(hidden)]
    #[visit(skip)]
    pub movement_right:             bool,

    // ? Block player movement if objects are in the pathway.
    #[reflect(hidden)]
    #[visit(skip)]
    pub movement_forward_block:     bool,
    #[reflect(hidden)]
    #[visit(skip)]
    pub movement_backward_block:    bool,
    #[reflect(hidden)]
    #[visit(skip)]
    pub movement_left_block:        bool,
    #[reflect(hidden)]
    #[visit(skip)]
    pub movement_right_block:       bool,

    // ? *Interact Button*, so that cursor-pointing activations may occur.
    #[reflect(hidden)]
    #[visit(skip)]
    pub do_interact:                bool,

    // ? Player camera perspective.
    pub perspective:                perspective::PlayerPerspective,
    pub camera:                     Handle<Node>,

    // ? Accept/ignore input if game window is focused/unfocused.
    pub input_focus:                bool,   // Whether game window has focus from the player.
    pub input_block:                bool,   // Block input for keybinding or GUI?

    // ? Player model (in case we want 3rd person view...)
    pub playermodel:                Handle<Node>,

}

impl Player {

    #[instrument(skip(resource_manager))]
    pub async fn new(scene: &mut Scene, resource_manager: &ResourceManager) -> Self {
        
        // Set up *skybox Future*, camera, perspective and rigid body model.
        let skybox = skybox::request(resource_manager);
        let camera: Handle<Node>;
        let perspective = perspective::PlayerPerspective::default();
        let playermodel = fyrox::scene::rigidbody::RigidBodyBuilder::new(

            fyrox::scene::base::BaseBuilder::new()
            .with_children(&[
                {
                    camera = CameraBuilder::new(
                        scene::base::BaseBuilder::new()
                    )
                    .with_skybox(skybox.await)
                    .with_fov(perspective.fov().unwrap()) // * FOV should always be `FirstPerson` by default.
                    .build(&mut scene.graph);
                    camera
                },
                // Add capsule collider for the rigid body.
                scene::collider::ColliderBuilder::new(scene::base::BaseBuilder::new())
                .with_shape(scene::collider::ColliderShape::capsule_y(0.25, 0.2))
                .build(&mut scene.graph)
            ])
            
        )
            .with_locked_rotations(true)
            .with_can_sleep(false)
            .build(&mut scene.graph);

        // Other defaults are also initialized.
        Player {
            camera,
            playermodel,
            ..Default::default()
        }

    }

}

impl ScriptTrait for Player {

    #[instrument(skip(context))]
    fn on_update(&mut self, context: &mut ScriptContext) {

        // ? Let dev know this scene is missing Player script node attachments...
        if self.camera.is_none() || self.playermodel.is_none() {
            error!(
                "This scene's Player script has missing nodes: {:?} - {:?}",
                self.camera,
                self.playermodel
            );
        }

        // TODO: Document.
        context.scene.graph[self.camera].local_transform_mut().set_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.perspective.pitch().to_radians()),
        );

        // TODO: Document.
        let body = context.scene.graph[self.playermodel]
            .as_rigid_body_mut();

        // TODO: Document.
        let mut velocity = Vector3::new(0.0, body.lin_vel().y, 0.0);
 
        // Move playermodel camera gimbal across floorplane when activated controls call for it.
        if self.movement_backward {
            velocity -= body.look_vector();
        } if self.movement_forward {
            velocity += body.look_vector();
        } if self.movement_left {
            velocity += body.side_vector();
        } if self.movement_right {
            velocity -= body.side_vector();
        }

        body.set_lin_vel(velocity);

        // TODO: Document.
        body.local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                self.perspective.yaw().to_radians()
            ));


    }

    #[instrument(skip(context))]
    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        
        // Run this code when the graphics context is initialized.
        with_igc(context.graphics_context, |igc| {

            // Grab cursor on focus.
            if self.input_focus {
                igc.window.set_cursor_grab(fyrox::window::CursorGrabMode::Confined)
                // Some platforms have no support for cursor grab.
                .or_else(|_e| igc.window.set_cursor_grab(fyrox::window::CursorGrabMode::Locked));
            }

            // Release cursor when focus is lost.
            else {
                igc.window.set_cursor_grab(fyrox::window::CursorGrabMode::None);
            }

            // Enable/disable cursor depending on whether we block input.
            igc.window.set_cursor_visible(self.input_block);

        });

        match event {
            
            Event::WindowEvent { window_id, event } => {

                // Enable/disable input if the game window is focused/unfocused.
                if let WindowEvent::Focused(focus) = event {

                    // Controls how *we* see window focus.
                    self.input_focus = focus.clone();
                    self.input_block = focus | true;

                }

                // Cursor entering window means player wants to direct their pointer at game.
                if let WindowEvent::CursorEntered { device_id } = event {
                    self.input_block = false;
                    self.input_focus = true; // TODO: Maybe remove, test and debug this pls.
                }

                // Cursor leaving window means player wants to direct their focus elsewhere.
                if let WindowEvent::CursorLeft { device_id } = event {
                    self.input_block = true;
                    self.input_focus = false;
                }

                // Keyboard input.
                if let WindowEvent::KeyboardInput { event, .. } = event {

                    // Pressed keys are mapped to movement modifiers.
                    if 
                        event.state.is_pressed() && // Only *keypresses* are read.
                        self.input_focus &&         // Window must have focus.
                        !self.input_block           // Game must not block input.
                    {
                        if let PhysicalKey::Code(keycode) = event.physical_key {
                            println!("Key {:?}", keycode);
                            match keycode {
                                
                                KeyCode::Escape => {
                                    self.input_focus = false;
                                    self.input_block = true;
                                },

                                KeyCode::KeyW => self.movement_forward = true,
                                KeyCode::KeyA => self.movement_left = true,
                                KeyCode::KeyS => self.movement_backward = true,
                                KeyCode::KeyD => self.movement_right = true,
                                KeyCode::KeyE => self.do_interact = true,
                                _ => {}
                            };
                        };
                    };

                }

                // Mouse wheel input.
                if let WindowEvent::MouseWheel { delta, .. } = event {

                }

            },

            Event::DeviceEvent { event, .. } => {

                // Device events are only accepted if we have window focus from the player.
                if self.input_focus && !self.input_block {
                    // Raw Mouse movements move the camera.
                    if let DeviceEvent::MouseMotion { delta } = event {
                        self.perspective.set_yaw( self.perspective.yaw() - delta.0 as f32);
                        self.perspective.set_pitch((self.perspective.pitch() + delta.1 as f32)
                            .clamp(-90.0, 90.0));
                    }
                }

            },

            _   => {},

        }

    }

    #[instrument(skip(context))]
    fn on_start(&mut self, #[allow(unused_variables)] context: &mut ScriptContext) {
        
        let gameplugin = context.plugins.get_mut::<crate::Game>();
        let scene = gameplugin.scene;

    }

}
