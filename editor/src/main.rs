
//! Editor with your game connected to it as a plugin.
use fyrox::{event_loop::EventLoop, gui::inspector::editors::{enumeration::EnumPropertyEditorDefinition, inspectable::InspectablePropertyEditorDefinition}};
use fyroxed_base::{Editor, StartupData};
use game:: {
    player::{ Player, perspective::PlayerPerspective, health::PlayerHealth },
    Game
};


fn main() {

    let event_loop = EventLoop::new().unwrap();
    let mut editor = Editor::new(
        Some(StartupData {
            working_directory: Default::default(),
            scenes: vec!["data/scene.rgs".into()],
        }),
    );
    
    editor.inspector.property_editors.insert(InspectablePropertyEditorDefinition::<Player>::new());
    editor.inspector.property_editors.insert(EnumPropertyEditorDefinition::<PlayerPerspective>::new());
    editor.inspector.property_editors.insert(InspectablePropertyEditorDefinition::<PlayerHealth>::new());
    editor.add_game_plugin(Game::default());
    editor.run(event_loop)

}
