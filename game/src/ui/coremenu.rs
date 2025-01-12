
//! ## CoreMenu
//! 
//! Game menu functionality. *CoreMenu* is both the *start* and *pause menu*.
//! 
//! TODO
//! 

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


/// Centralized menu system.
#[derive(Debug, Visit, Reflect)]
pub struct CoreMenuUI {

    /// Menu visibility with lockable state system.
    #[reflect(hidden)]
    #[visit(skip)]
    pub visibility:         CoreMenuVisibility,

    // /// Menu mode
    // /// 
    // /// Transforms conveyed menu content as game starts, plays and quits.
    // #[reflect(hidden)]
    // #[visit(skip)]
    // pub mode:               CoreMenuMode, // TODO: Write this.

    #[reflect(hidden)]
    #[visit(skip)]
    pub button_quit:        Handle<gui::UiNode>,

    #[reflect(hidden)]
    #[visit(skip)]
    pub button_settings:    Handle<gui::UiNode>,

    #[reflect(hidden)]
    #[visit(skip)]
    pub button_resume:      Handle<gui::UiNode>,

    /// Game Settings
    /// 
    /// Always-available settings access.
    #[reflect(hidden)]
    #[visit(skip)]
    pub ui_settings:        settings_ui::SettingsPane,

}

impl CoreMenuUI {
    
    pub fn new(context: &mut PluginContext) -> Self {
        
        let visibility = CoreMenuVisibility::default();
        let button_quit;
        let button_settings;
        let button_resume;
        let ui_settings = settings_ui::SettingsPane::default();

        let ui = context.user_interfaces.first_mut();
        let mut context = ui.build_ctx();

        // Screen size.
        gui::screen::ScreenBuilder::new(
            gui::widget::WidgetBuilder::new().with_child(
                // CoreMenu pause/settings layout grid.
                gui::grid::GridBuilder::new(
                    gui::widget::WidgetBuilder::new()

                        // Resume button.
                        .with_child({
                            button_resume = gui::button::ButtonBuilder::new(
                                gui::widget::WidgetBuilder::new().on_column(0).on_row(0)
                            )
                                .with_text("Resume")
                                .build(&mut context);
                            button_resume
                        })

                        // Settings button.
                        .with_child({
                            button_settings = gui::button::ButtonBuilder::new(
                                gui::widget::WidgetBuilder::new().on_column(0).on_row(0)
                            )
                                .with_text("Settings")
                                .build(&mut context);
                            button_settings
                        })

                        // Quit button.
                        .with_child({
                            button_quit = gui::button::ButtonBuilder::new(
                                gui::widget::WidgetBuilder::new().on_column(0).on_row(0)
                            )
                                .with_text("Quit")
                                .build(&mut context);
                            button_quit
                        })

                )
                    .add_columns(vec![
                        // One column per row.
                        gui::grid::Column::auto()
                    ])
                    .add_rows(vec![
                        // Two rows, one with sub-grid-contained menu buttons `Resume`, `Settings`, `Quit`...
                        // Other row contains a sub-grid with settings pane.
                        gui::grid::Row::auto(),
                        gui::grid::Row::auto()
                    ])
                    .build(&mut context)
            )
        ).build(&mut context);

        CoreMenuUI {
            visibility,
            // mode,
            button_quit,
            button_settings,
            button_resume,
            ui_settings
        }

    }

}

#[test]
fn test_coremenu_visibility() {

    // Start with `Show`.
    let mut cmv = CoreMenuVisibility::Show;

    // Testing `is_*` functionality...
    assert_eq!(cmv.is_show(), true);                // IS shown!
    assert_eq!(cmv.is_hide(), false);               // Not hidden!
    assert_eq!(cmv.is_locked(), false);             // Not locked!

    // Changing the `Show`, `Hide` and `Locked` states.
    cmv.hide();                                     // Now hide.
    assert_eq!(cmv.is_show(), false);               // Not shown!
    assert_eq!(cmv.is_hide(), true);                // IS hidden!
    assert_eq!(cmv.is_locked(), false);             // Not locked!

    // Changing the `Locked` state and testing its functions...
    cmv.lock(true);                           // Now lock to show.
    assert_eq!(cmv.is_locked_with(true), true);     // Must be type `Locked` with `show: true`.
    cmv.lock_hidden();                              // Now lock to hide.
    assert_eq!(cmv.is_locked_with(false), true);    // Must be type `Locked` with `show: false`.
    cmv.lock_shown();                               // Now lock to show again.

    // Let's see how `.is_locked_and(...)` functions...
    assert_eq!(cmv.is_locked_and().unwrap(), true);
    cmv.lock_hidden();
    assert_eq!(cmv.is_locked_and().unwrap(), false);

    // Return to `Show` and `Hide` testing for final toggle test...
    cmv.hide();
    cmv.toggle();                                   // Toggles to `Show`.
    assert_eq!(cmv.is_show(), true);                // Must be type `Show`.

    // Congrats! Your type works. 😼

}

/// Indicates visibility of the menu to the render stack and allows '(un-)locking' of the state.
#[derive(Debug, Eq, PartialEq, Default)]
pub enum CoreMenuVisibility {

    /// Show `CoreMenu`.
    Show,

    /// Hide `CoreMenu`.
    #[default]
    Hide,

    /// Lock `CoreMenu` to show/hide depending on the value in `show`.
    /// The player doesn't have the ability to dismiss or show the menu when it's locked to either state.
    Locked {
        /// Lock the menu to show/hide.
        show:   bool,
    }

}

impl CoreMenuVisibility {

    /// Show the menu.
    pub fn show(&mut self) {
        *self = Self::Show;
    }

    /// Hide the menu.
    pub fn hide(&mut self) {
        *self = Self::Hide;
    }

    /// Lock the menu to `show`.
    pub fn lock(&mut self, show: bool) {
        *self = Self::Locked { show };
    }

    /// Convenience function of `.lock(true)` to show CoreMenu.
    pub fn lock_shown(&mut self) {
        self.lock(true);
    }

    /// Convenience function of `.lock(false)` to hide CoreMenu.
    pub fn lock_hidden(&mut self) {
        self.lock(false);
    }

    /// Toggle menu unless it's locked.
    pub fn toggle(&mut self) {
        match self {
            CoreMenuVisibility::Show    => self.hide(),
            CoreMenuVisibility::Hide    => self.show(),
            _                           => {}, // Do nothing with locked self.
        }
    }

    /// Is menu shown?
    /// 
    /// Returns `true` if shown, and `false` for hidden.
    pub fn is_show(&self) -> bool {
        match self {
            CoreMenuVisibility::Show            => true,
            _                                   => false,
        }
    }

    /// Is menu hidden?
    /// 
    /// Returns `true` if hidden, and `false` for shown.
    pub fn is_hide(&self) -> bool {
        match self {
            CoreMenuVisibility::Hide            => true,
            _                                   => false,
        }
    }

    /// Is menu locked?
    /// 
    /// Returns `true` if locked, and `false` for not locked.
    /// ***This function doesn't indicate if the menu is hidden/shown!***
    pub fn is_locked(&self) -> bool {
        match self {
            CoreMenuVisibility::Locked { .. }   => true,
            _                                   => false,
        }
    }

    /// Determine if `self` is locked with value `eq_show`.
    /// 
    /// Tests whether:
    ///     1. `self` is locked to a particular value, and,
    ///     2. `self` equals `eq_show`, in which case returns `true`.
    /// 
    /// If `self` is not locked *or* `self` is not equal with `eq_show`, then `false` is returned.
    /// 
    /// For an equivalent function that explicitly indicates if `self` is locked, see `.is_locked_and()`.
    pub fn is_locked_with(&self, eq_show: bool) -> bool {
        match self {
            CoreMenuVisibility::Locked { show } => {
                if show == &eq_show { true }
                else                { false }
            },
            _                                   => false,
        }
    }

    /// Determine if `self` is locked and if so, return the inner value.
    /// 
    /// When `self` is locked, the returned Option type is `Some(bool)` containing the boolean value indicating whether
    /// CoreMenu is shown or not. If `self` is not locked, Option type `None` is returned.
    pub fn is_locked_and(&self) -> Option<bool> {
        if let Self::Locked { show } = self {
            Some(show.clone())
        }
        else {
            None
        }
    }

}

/// A set of messages related to `CoreMenu`.
#[derive(Debug)]
pub enum CoreMenuMessage {

    /// Pause the game, show `CoreMenu`.
    Pause,

    /// Unpause the game, hide `CoreMenu`.
    Resume,

    /// Load Scene file.
    LoadSceneFile(std::path::PathBuf),

    /// Show the *Settings Pane*.
    SettingsPaneOpen,

    /// Close the *Settings Pane*.
    SettingsPaneClose,

    /// Quit the game.
    Quit

}

// TODO: Change module name to something else.
pub mod settings_ui {

    /// Settings Menu
    #[derive(Debug, Default)]
    pub struct SettingsPane {
    
    
    
    }

}
