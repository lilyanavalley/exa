
use serde:: { Serialize, Deserialize };
use super::ui::SerializedColor;


/// Dialog Speaker.
/// 
/// Configuration for a speaking role in dialog. This represents one character or other entity which delivers dialog 
/// lines. Every speaker is assumed to be unique from a technical perspective, but this limitation does not extend to 
/// development use.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DialogSpeaker {

    // ? A speaker for dialog.

    /// Color of the speaker's name.
    color:              SerializedColor,

    /// Internally-referenced name.
    key:                String,

    /// An FTL message key to reference for the name of this speaker.
    ftl_key_name:       String,

    /// Visual representation of the dialog window to use for this speaker.
    kind:               SpeakerKind,

    speakrate:          Option<f32>,
    speakrate_pause:    Option<f32>,

}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
enum SpeakerKind {

    #[default]
    /// Comes from an NPC.
    Npc,

    /// Comes from the player.
    Player,

    /// Comes from the game.
    Game,

    /// A barren dialog window is presented instead of personalized to the speaker's preferences.
    Unidentified,

}
