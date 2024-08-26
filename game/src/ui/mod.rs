
use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*
    },
    gui,
    engine::InitializedGraphicsContext,
    plugin::{ Plugin, PluginContext },
};
use tracing::instrument;


pub mod developer;


#[derive(Debug, Reflect, Visit)]
pub struct UiSubset {

    /// Developer Overlay
    #[reflect(hidden)]
    #[visit(skip)]
    pub developer_overlay:  Option<developer::DeveloperOverlay>,

    // #[reflect(hidden)]
    // #[visit(skip)]
    // _tracy_framebuffer:     Option<renderer::framework::framebuffer::FrameBuffer>,

}

impl UiSubset {

    pub fn new(plugin: &mut PluginContext) -> Self {

        // TODO: Perform UI setup routine here.
        UiSubset::default()

    }

    pub fn developeroverlay_set(&mut self, show: bool, plugin: &mut PluginContext) {
        if show {
            self.developer_overlay = Some(developer::DeveloperOverlay::new(plugin));
        }
        else {
            self.developer_overlay = None;
        }
    }

    #[instrument(name = "UI Update", skip(context))]
    pub fn update(&mut self, context: &mut PluginContext) {
        
        // Update the Developer Overlay, if it is shown.
        if let Some(developer_overlay) = &mut self.developer_overlay {
            
            let ui_first = context.user_interfaces.first();
            let igc = context.graphics_context.as_initialized_ref();

            // Delta counter.
            let dt = context.dt;
            ui_first.send_message(gui::text::TextMessage::text(
                developer_overlay.dt,
                gui::message::MessageDirection::ToWidget,
                format!("Delta {}", dt)
            ));

            let fps = igc.renderer.get_statistics().frames_per_second;
            ui_first.send_message(gui::text::TextMessage::text(
                developer_overlay.fps,
                gui::message::MessageDirection::ToWidget,
                format!("FPS {}", fps)
            ));

        }

    }

}

impl Default for UiSubset {
    fn default() -> Self {
        UiSubset {
            developer_overlay:      None,
        }
    }
}
