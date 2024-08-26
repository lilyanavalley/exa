
use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*
    },
    plugin:: { PluginContext },
    gui,
    gui::UiNode
};


/// Developer Overlay
/// 
/// Display Developer-oriented information about the engine state and performance.
#[derive(Debug, Reflect, Visit)]
pub struct DeveloperOverlay {

    // /// Show Developer Overlay?
    // /// 
    // /// TODO: Document.
    // pub show:       bool,

    /// FPS Counter.
    /// 
    /// TODO: Document.
    pub fps:        Handle<UiNode>,

    /// Delta Time.
    /// 
    /// TODO: Document.
    pub dt:         Handle<UiNode>,

    /// Lag time.
    /// 
    /// TODO: Document.
    pub lag:        Handle<UiNode>,

}

impl DeveloperOverlay {

    pub fn new(plugin: &mut PluginContext) -> Self {

        // Using these later to build widgets.
        let ui = plugin.user_interfaces.first_mut();
        let context = &mut ui.build_ctx();

        // These are fields of `DeveloperOverlay.`
        let fps;
        let dt;
        let lag;

        // Screen widget gives us the bounds of the screen to work inside of...
        gui::screen::ScreenBuilder::new(
            gui::widget::WidgetBuilder::new().with_child(
                // This grid contains a few columns with information about the game's inner state and performance.
                gui::grid::GridBuilder::new(
                    gui::widget::WidgetBuilder::new()
        
                        // FPS counter.
                        .with_child({
                            fps = gui::text::TextBuilder::new(
                                gui::widget::WidgetBuilder::new().on_column(0).on_row(0)
                            )
                                .with_text("FPS")
                                .with_font_size(32.0)
                                .build(context);
                            fps
                        })
        
                        // Engine update rate.
                        .with_child({
                            dt = gui::text::TextBuilder::new(
                                gui::widget::WidgetBuilder::new().on_column(1).on_row(0)
                            )
                                .with_text("Delta")
                                .with_font_size(32.0)
                                .build(context);
                            dt
                        })
        
                        // Lag time.
                        .with_child({
                            lag = gui::text::TextBuilder::new(
                                gui::widget::WidgetBuilder::new().on_column(2).on_row(0)
                            )
                                .with_text("Lag")
                                .with_font_size(32.0)
                                .build(context);
                            lag
                        })
        
                )
                    .add_column(gui::grid::GridDimension::auto())
                    .add_column(gui::grid::GridDimension::auto())
                    .add_column(gui::grid::GridDimension::auto())
                    .add_row(gui::grid::GridDimension::strict(42.0))
                    .draw_border(true)
                    .with_border_thickness(1.0)
                .build(context)

            )
        ).build(context);


        DeveloperOverlay {
            fps,
            dt,
            lag
        }
    
    }

    // pub fn show(&self) -> bool {
    //     self.show
    // }

    // pub fn set_show(&mut self, enabled: bool) {
    //     self.show = enabled;
    // }

}

impl Default for DeveloperOverlay {
    fn default() -> Self {
        DeveloperOverlay {
            fps:        Handle::NONE,
            dt:         Handle::NONE,
            lag:        Handle::NONE,
        }
    }
}

