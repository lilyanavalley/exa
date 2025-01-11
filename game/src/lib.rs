
// This file is part of EXA.
// EXA is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as 
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// EXA is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with EXA. If not, see 
// <https://www.gnu.org/licenses/>.

//!
//! Game project.
//! 
//! TODO
//! 


pub mod player;     /// Player object and script.
pub mod settings;   /// Player/game settings.
pub mod ui;         /// Game User Interface.
// pub mod eventline;  /// Events processor subroutine.
mod utilities;      /// Game utilities.
mod components;     /// Engine components.

#[cfg(feature = "tracy")]
mod tracy;          /// Tracy utilities and subroutines.


use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*
    }, event::{DeviceEvent, Event}, gui::{ message:: { MessageDirection, UiMessage }, text::{Text, TextMessage}, UiNode }, keyboard:: { PhysicalKey, KeyCode }, plugin::{ Plugin, PluginContext, PluginRegistrationContext }, scene::Scene
};
use std:: { future::{Future, IntoFuture}, path::Path };
use tracing:: { trace, trace_span, debug, debug_span, info, info_span, warn, warn_span, error, error_span, instrument::Instrument, instrument };
use crate::utilities::*;

#[cfg(feature = "tracy")]
use { tracy_client, tracy_client_sys };

/// Game title.
const GAME_TITLE:       &'static str    = "Experiment A";
/// Game version is read in package file `Cargo.toml`.
const GAME_VERSION:     &'static str    = env!("CARGO_PKG_VERSION");
/// Save file path.
const SAVE_FILEPATH:    &'static str    = "./save0.bin";


#[derive(Debug, Reflect, Visit, Default)]
pub struct Game {

    /// Active Scene.
    scene:          Handle<Scene>,

    /// User Interfaces.
    ui:             ui::UiSubset,

    /// Game Settings and the Settings UI.
    #[visit(skip)]
    #[reflect(hidden)]
    settings:       settings::Settings,

    #[visit(skip)]
    #[reflect(hidden)]
    gamepads:       Option<gilrs::Gilrs>,

    #[visit(skip)]
    #[reflect(hidden)]
    localization:   components::fluent::FluentCache,

    #[visit(skip)]
    #[reflect(hidden)]
    dialog:         components::dialog::Dialog,

}

impl Game {

    // #[instrument(skip(context))]
    // pub fn new(scene_path: Option<&str>, mut context: PluginContext) -> Self {

    //     trace!("Creating game plugin context...");

    //     context
    //         .async_scene_loader
    //         .request(scene_path.unwrap_or("data/scene.rgs"));

    //     Self {
    //         scene:      Handle::NONE,
    //         ui:         ui::UiSubset::new(&mut context),
    //         settings:   settings::Settings::default()
    //     }
    // }

    fn save(&self, context: &mut PluginContext<'_, '_>) -> VisitResult {

        let mut visitor = Visitor::new();
        context.scenes[self.scene].save("Scene", &mut visitor)?;
        // TODO: Add game fields as necessary.
        visitor.save_binary(SAVE_FILEPATH)

    }

    fn load(&mut self, context: &mut PluginContext<'_, '_>) {
        context.async_scene_loader.request_raw(SAVE_FILEPATH);
    }

    // TODO: Document.
    fn on_suspended(&self) {

        // TODO: Pause game.

    }

    // TODO: Document.
    fn on_resumed(&self) {

        // TODO: Continue game.

    }

    // TODO: Document.
    fn on_loopexiting(&self) {

        // TODO: Deinit things and run save routines where necessary.

    }

    // TODO: Document.
    fn on_memory_warning(&self) {
        
        // TODO: Consider lessening the memory footprint wherever possible.
        // ? This doesn't do much besides warn the application user that the app may be terminated soon.
        error!("Memory Warning: OS indicates memory usage exceeds limits and may terminate this game soon!");

    }

}

impl Plugin for Game {

    #[instrument(skip(_context))]
    fn on_deinit(&mut self, _context: PluginContext) {
        
    }

    #[instrument(
        name = "Plugin Update",
        skip(context, self)
    )]
    fn update(&mut self, context: &mut PluginContext) {

        // Run gamepad input updates.
        if let Some(gilrs) = &mut self.gamepads {

            while let Some(event) = gilrs.next_event() {
                trace!("gilrs event by {id}: {ev:?}", id = event.id, ev = event.event);
            };

            // Increment event counter.
            // * This must be called after all event processing with **Gilrs**!
            gilrs.inc();
        }

        // Run UI updates.
        self.ui.update(context);

        // Retrieve initialized graphics context for updating.
        if let fyrox::engine::GraphicsContext::Initialized(igc) = context.graphics_context {

            #[cfg(feature="tracy")]
            {
                // If Tracy is running, collect a frame image.
                let _tracy = tracy_client::Client::running();
                if _tracy.is_some() {
                    let _tracy = _tracy.unwrap();
                    tracy::frameimage_collect(igc);
                    _tracy.frame_mark();
                }
            }

        }

    }

    #[instrument(skip(_context))]
    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        mut _context: PluginContext,
    ) {

        match _event {

            Event::NewEvents(cause) => {
                // For reading cause of incoming events.
            },

            Event::WindowEvent { 
                window_id, event
            }                       => {

            },

            // TODO: Remove after testing load/save functionality.
            Event::DeviceEvent { device_id, event } => {
                if let DeviceEvent::Key(key) = event {
                    
                    // Performs a save/load on F2/F3 key press.
                    if let PhysicalKey::Code(kc) = key.physical_key {
                        match kc {
                            
                            KeyCode::F2     => {
                                info!("Saving game; F2 key...");
                                self.save(&mut _context);
                            },

                            KeyCode::F3     => {
                                info!("Loading game; F3 key...");
                                self.load(&mut _context);
                            },

                            _               => {}

                        }
                    }

                }
            },

            // Event::UserEvent(_)     => todo!(),

            Event::Suspended        => self.on_suspended(),

            Event::Resumed          => self.on_resumed(),

            // Event::AboutToWait      => todo!(),

            Event::LoopExiting      => self.on_loopexiting(),

            Event::MemoryWarning    => self.on_memory_warning(),

            _ => {} // Ignore other event types.

        }
        
    }

    #[instrument(skip(_context))]
    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
    ) {
        // Handle UI events here.
    }

    #[instrument(skip(_context))]
    fn on_scene_begin_loading(&mut self, path: &Path, _context: &mut PluginContext) {

        //? Paths for a scene *file* should never be a directory.
        // This function doesn't really do much besides indicate the given path is probably causing an error elsewhere.
        if path.is_dir() {
            error!("Scene file provided appears to be a directory instead: {}", path.display());
        };

        //? When we're loading a scene, the *current* scene we're playing will need to be removed.
        if self.scene.is_some() {
            _context.scenes.remove(self.scene);
        }

    }

    #[instrument(skip(context, data))]
    fn on_scene_loaded(
        &mut self,
        path: &Path,
        new_scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) {

        // Report and set the scene into `self`.
        info!("Scene ({scene:?}) loaded: {path}", scene = new_scene, path = path.display());
        self.scene = new_scene;
        if let Ok(mut visitor)= Visitor::load_from_memory(data) {
            // TODO: Take `data` and visit `self` fields.
        }

    }
    
    #[instrument(skip(context))]
    fn register(&self, context: PluginRegistrationContext) {

        // TODO: Register scripts here.
        let script = &context.serialization_context.script_constructors;
        script.add::<player::Player>("Player");

    }
    
    #[instrument]
    fn register_property_editors(&self) -> fyrox::gui::inspector::editors::PropertyEditorDefinitionContainer 
    {
        fyrox::gui::inspector::editors::PropertyEditorDefinitionContainer::empty()
    }
    
    #[instrument(skip(context, self))]
    fn init(
        &mut self,
        scene_path: Option<&str>,
        mut context: PluginContext,
    ) {

        // ? Loads the development scene.
        // TODO: Remove or replace.
        context.async_scene_loader.request(scene_path.unwrap_or("data/scene.rgs"));

        // ? Starts plugin task to fetch US English fluent file.
        let trace_defaultftlfetch = trace_span!("FTL default fetch Future");
        context.task_pool.spawn_plugin_task(
            components::fluent::FluentCache::default_later()
                .into_future()
                .instrument(trace_defaultftlfetch),
            | data, game: &mut Game, _context | {
                game.localization.bundle = data.unwrap(); // TODO: Replace .unwrap for stability.
                trace!("End FTL default fetch.");
            }
        );

        self.ui = ui::UiSubset::new(&mut context);

        // Attempt to retrieve gamepads from the system.
        if let Ok(gilrs) = gilrs::Gilrs::new() {
            
            // ? Indicate what gamepads are connected.
            for (each_gamepad_id, _each_gamepad) in gilrs.gamepads() {
                info!("connected gamepad: {}", each_gamepad_id);
            }

            self.gamepads = Some(gilrs);

        }

    }
    
    #[instrument(skip(_context))]
    fn on_loaded(&mut self, _context: PluginContext) {

    }
    
    #[instrument(skip(context))]
    fn on_graphics_context_initialized(
        &mut self,
        context: PluginContext,
    ) {

        trace!("Graphics context initialized!");
        // // TODO: Register tracy framecollector render pass.
        // utilities::with_igc(context.graphics_context, |igc| {
            
        //     let rc_tracy_framecollecter = std::rc::Rc::new(std::cell::RefCell::new(tracy::FrameCollector::default()));

        //     igc.renderer.add_render_pass(rc_tracy_framecollecter);
        // });

    }
    
    #[instrument(skip(_context))]
    fn before_rendering(&mut self, _context: PluginContext) {

    }
    
    #[instrument(skip(_context))]
    fn on_graphics_context_destroyed(&mut self, _context: PluginContext) {
        trace!("Graphics context destroyed!");
    }
    
    fn on_scene_loading_failed(
        &mut self,
        #[allow(unused_variables)] path: &Path,
        #[allow(unused_variables)] error: &fyrox::core::visitor::prelude::VisitError,
        #[allow(unused_variables)] context: &mut PluginContext,
    ) {
        error!("Scene could not be loaded: {path:?} ({error:?})");
    }

}
