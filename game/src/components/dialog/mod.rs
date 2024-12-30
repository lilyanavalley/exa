
use std:: {
    collections::BTreeMap,
};
use fluent_bundle::FluentMessage;
use fyrox:: {
    core::{ 
        futures::channel::mpsc::{ Receiver, Sender },
        pool::Handle, reflect::prelude::*, visitor::prelude::*,
        type_traits::prelude::*, 
        color::Color,
        TypeUuidProvider
    },
    script::{ ScriptContext, ScriptMessage, ScriptTrait, BaseScript },
    scene::node::Node
};
use serde:: { Serialize, Deserialize };
use ui::UiVisibility;

pub mod speaker;
pub mod conversation;
pub mod ui;
mod error;


type SpeakerHandle = String; // Flat pointer to a particular `DialogSpeaker`.
type ConversationHandle = String;  // Flat pointer to a particular `Conversation`.

/// Dialog Component
/// 
/// todo: document.
#[derive(Debug, Default)]
pub struct Dialog {

    // ? manages the internal dialog system and recalls conversations, speakers and routes which are taken in speech.

    /// User Interface Visibility.
    pub visibility: ui::UiVisibility,

    /// Active Dialog Ticket. `Option<...>` returns the ticket for the script., if one exists... 
    pub ticket:     Option<Ticket>,

    // Cache of conversations and their turns. 
    cache_convos:   BTreeMap<ConversationHandle, conversation::Conversation>,

    // Cache of speakers ('characters') in a conversation.
    cache_speakers: BTreeMap<SpeakerHandle, speaker::DialogSpeaker>

}

impl Dialog {

    /// Alias to `Dialog::default()`.
    pub fn new() -> Self {
        Dialog::default()
    }

    // TODO: Document.
    pub fn ticket(&self) -> &Option<Ticket> {
        &self.ticket
    }

    // TODO: Document.
    pub fn ticket_load(&mut self, id: &str) -> Result<usize, error::DialogError> {
        Ok(0)
    }

    /// Retrieve a conversation from the cache by its handle.
    pub fn conversation(&self, handle: &ConversationHandle) -> Option<&conversation::Conversation> {
        self.cache_convos.get(handle)
    }

    // TODO: Document.
    pub fn conversation_exists(&self, handle: &ConversationHandle) -> bool {
        self.cache_convos.contains_key(handle)
    }

    /// Retrieve a speaker from the cache by its handle. 
    pub fn speaker(&self, handle: &SpeakerHandle) -> Option<&speaker::DialogSpeaker> {
        self.cache_speakers.get(handle)
    }

    // TODO: Document.
    pub fn speaker_exists(&self, handle: &SpeakerHandle) -> bool {
        self.cache_speakers.contains_key(handle)
    }

}

/// Dialog Ticket
/// 
/// A 'ticket' to in-progress conversation.
/// 
/// Contains the `id` and `state` of this ticket, in reference to a more fully-defined `DialogConvo`.
#[derive(Debug, Default)]
pub struct Ticket {

    id:         String,
    state:      TicketState

}

/// State of `Ticket`.
#[derive(Debug, Default)]
pub enum TicketState {

    /// Nothing is happening.
    #[default]
    Paused,

    /// The dialog system is being initialized to (soon) show dialog to the player.
    Initializing,

    /// The dialog system is currently spitting dialog out to the player.
    Dictating,

    /// We're waiting for the player to make a dialog choice selection.
    WaitingForPlayerChoice,

    /// Dialog is being cancelled by the player.
    Cancelling,

}

/// Interactive Dialog Point.
/// 
/// Accepts 'interactions' and calls upon the Dialog component to prepare and deliver dialog as a character.
#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "1e9befc4-fac0-4a3c-9140-d88f6014ae7b")]
#[visit(optional)]
pub struct DialogPoint {
    

    model:  Handle<Node>,

}

impl DialogPoint {

    /// Alias to `InteractiveDialog::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    // /// Receive messages from the message queue.
    // pub fn message_receiver(&self) -> &Receiver<InteractiveDialogMessage> {
    //     self.message_receiver
    // }

}

impl ScriptTrait for DialogPoint {

    fn on_init(&mut self, ctx: &mut ScriptContext) {
        
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        
    }

}

#[derive(Debug, Default)]
pub enum InteractiveDialogPointMessage {

    /// A message about the current dialog state, from subtype `TicketState`.
    Ticket(TicketState),

    /// A message about the Dialog UI, from subtype `ui::UiVisibility`.
    Ui(UiVisibility),

    /// Reset cached conversations and speakers to default state.
    CachesInvalidate,

    /// Populate caches from their resources.
    /// TODO: Implement functionality to request and load specific resources, discarding the rest.
    CachesReload,

    /// While Dialog UI is on-screen and `RCM` message is received, should *CoreMenu* be allowed to take over?
    /// This prevents certain dialog from pausing the game under normal circumstances, represented by the variable
    /// `yielding_to_pause`, with `true` for *allowing to pause* and `false` for *will not pause*.
    /// TODO: Document this passage better.
    RequestCoreMenu { yielding_to_pause: bool },

    #[default]
    /// Dialog component is busy collecting resources or doing calculations and is unavailable at the moment.
    Busy,

}

mod tests {
    
    use super::{ Dialog, ui::UiVisibility };


    #[test]
    fn test_dialog() {

        let mut dialog = Dialog::new();
        assert!(dialog.cache_speakers.is_empty());
        assert!(dialog.cache_convos.is_empty());
        assert!(dialog.ticket.is_none());
        // * There is a test for UI visibility. Please see `ui::tests::test_uivisibility_default()` and
        // * `ui::tests::test_uivisibility()`

        assert!(dialog.ticket().is_none());
        
        assert!(dialog.speaker(&"testspeaker".to_string()).is_none());
        assert_eq!(dialog.speaker_exists(&"testspeaker".to_string()), false);

        assert!(dialog.conversation(&"testconvo".to_string()).is_none());
        assert_eq!(dialog.conversation_exists(&"testconvo".to_string()), false);

    }

}
