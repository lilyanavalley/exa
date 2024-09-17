//!
//! Game project.
//! 
//! TODO
//! 


pub mod player;     /// Player object and script.
pub mod settings;   /// Player/game settings.
pub mod ui;         /// Game User Interface.
mod tracy;          /// Tracy utilities and subroutines.
mod utilities;      /// Game utilities.


use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*
    },
    event::Event,
    gui::{ message:: { MessageDirection, UiMessage }, text::{Text, TextMessage}, UiNode },
    plugin::{ Plugin, PluginContext, PluginRegistrationContext },
    scene::Scene
};
use std::path::Path;
use { tracy_client, tracy_client_sys };
use tracing:: { trace, trace_span, debug, debug_span, info, info_span, warn, warn_span, error, error_span, instrument };
use crate::utilities::*;


/// Game title.
const GAME_TITLE:       &'static str    = "Experiment A";
/// Game version is read in package file `Cargo.toml`.
const GAME_VERSION:     &'static str    = env!("CARGO_PKG_VERSION");


#[derive(Debug, Reflect, Visit, Default)]
pub struct Game {

    /// Active Scene.
    scene:          Handle<Scene>,

    /// User Interfaces.
    ui:             ui::UiSubset,

}

impl Game {

    #[instrument(skip(context))]
    pub fn new(scene_path: Option<&str>, mut context: PluginContext) -> Self {

        trace!("Creating game plugin context...");

        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        Self {
            scene:      Handle::NONE,
            ui:         ui::UiSubset::new(&mut context)
        }
    }

}

impl Plugin for Game {

    #[instrument(skip(_context))]
    fn on_deinit(&mut self, _context: PluginContext) {
        
    }

    #[instrument(
        name = "Plugin Update",
        skip(context)
    )]
    fn update(&mut self, context: &mut PluginContext) {

        // Retrieve Tracy client handle.
        let _tracy = tracy_client::Client::running();

        // Run UI updates.
        self.ui.update(context);

        // Retrieve initialized graphics context for updating.
        if let fyrox::engine::GraphicsContext::Initialized(igc) = context.graphics_context {

            // If Tracy is running, collect a frame image.
            if _tracy.is_some() {
                let _tracy = _tracy.unwrap();
                tracy::frameimage_collect(igc);
                _tracy.frame_mark();
            }

        }

    }

    #[instrument(skip(_context))]
    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
    ) {

        match _event {

            Event::NewEvents(cause) => {
                // For reading cause of incoming events.
            },

            // Event::WindowEvent { 
            //     window_id, event
            // }                       => {
            //     // Window events.
            // },

            // Event::DeviceEvent {
            //     device_id, event
            // }                       => todo!(),

            // Event::UserEvent(_)     => todo!(),

            // Event::Suspended        => todo!(),

            // Event::Resumed          => todo!(),

            // Event::AboutToWait      => todo!(),

            // Event::LoopExiting      => todo!(),

            // Event::MemoryWarning    => todo!(),

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

    }
    
    #[instrument(skip(context))]
    fn register(&self, context: PluginRegistrationContext) {

        // TODO: Register scripts here.

        // context
        //     .serialization_context
        //     .script_constructors
        //     .add::<Sequencer>("Sequencer");

    }
    
    #[instrument]
    fn register_property_editors(&self) -> fyrox::gui::inspector::editors::PropertyEditorDefinitionContainer 
    {
        fyrox::gui::inspector::editors::PropertyEditorDefinitionContainer::empty()
    }
    
    #[instrument(skip(context))]
    fn init(
        &mut self,
        scene_path: Option<&str>,
        mut context: PluginContext,
    ) {

        // ? Loads the development scene.
        // TODO: Remove or replace.
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        self.ui = ui::UiSubset::new(&mut context);

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
        // TODO: Register tracy framecollector render pass.

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
