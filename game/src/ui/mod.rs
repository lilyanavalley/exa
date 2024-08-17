
use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*
    },
    gui,
    plugin::{ Plugin, PluginContext },
    renderer
};
use tracing::instrument;


pub mod developer;


#[derive(Debug, Reflect, Visit)]
pub struct UiSubset {

    #[reflect(hidden)]
    #[visit(skip)]
    pub developer_overlay:  developer::DeveloperOverlay,

    // #[reflect(hidden)]
    // #[visit(skip)]
    // _tracy_framebuffer:     Option<renderer::framework::framebuffer::FrameBuffer>,

}

impl UiSubset {

    pub fn new(plugin: &mut PluginContext) -> Self {

        // let ui_first = plugin.user_interfaces.first();
        // let build_context = &mut ui_first.build_ctx();

        let developer_overlay = developer::DeveloperOverlay::new(plugin, true);

        UiSubset {
            developer_overlay
        }

    }

    pub fn developer_overlay(&self) -> &developer::DeveloperOverlay {
        &self.developer_overlay
    }

    pub fn set_developer_overlay(&mut self, enabled: bool) {
        self.developer_overlay.set_show(enabled);
    }

    #[instrument(name = "UI Update", skip(context))]
    pub fn update(&mut self, context: &mut PluginContext) {
        
        let ui_first = context.user_interfaces.first();

        // Update the Developer Overlay if it is shown.
        if self.developer_overlay.show() {
            
            // Delta counter.
            let dt = context.dt;
            ui_first.send_message(gui::text::TextMessage::text(
                self.developer_overlay.dt,
                gui::message::MessageDirection::ToWidget,
                format!("Delta {}", dt)
            ));

            // ui_first.send_message(gui::text::TextMessage::text(
            //     self.developer_overlay.fps,

            // ))

        }

    }

}

impl Default for UiSubset {
    fn default() -> Self {
        UiSubset {
            developer_overlay:      developer::DeveloperOverlay::default()
        }
    }
}
