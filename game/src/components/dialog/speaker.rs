
// This file is part of EXA.
// EXA is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as 
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// EXA is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with EXA. If not, see 
// <https://www.gnu.org/licenses/>.

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
