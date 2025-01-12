
// This file is part of EXA.
// EXA is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as 
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// EXA is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with EXA. If not, see 
// <https://www.gnu.org/licenses/>.

use serde:: { Serialize, Deserialize };


/// A single conversation.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conversation {

    /// Turns in a conversation.
    turns:  Vec<ConversationTurn>,

}

/// A single turn containing FTL messages for a particular character, their speaker handle and additional options.
#[derive(Debug, Default, Serialize, Deserialize)]
struct ConversationTurn {

    messages:   Vec<String>,    // TODO: reference fluent messages by their key
    speaker:    String,

    pausable:   Option<bool>,
    skippable:  Option<bool>,

}

impl ConversationTurn {



}

/// A builder for [`ConversationTurn`].
#[derive(Debug)]
struct ConversationTurnBuilder {

    inner: ConversationTurn

}

impl ConversationTurnBuilder {

    /// Create a new turn with the `speaker`'s key.
    fn new(speaker: String) -> Self {
        let mut builder = Self {
            inner:  ConversationTurn::default()
        };
        builder.inner.speaker = speaker;
        builder
    }

    /// Specify if turn should be pausable.
    /// 
    /// *Default is `false`, which indicates NOT pausable.*. `true` indicates pausable.
    fn with_pausable(mut self, pausable: bool) -> Self {
        self.inner.pausable = Some(pausable);
        self
    }

    /// Specify if turn should be skippable.
    /// 
    /// *Default is `false`, which indicates NOT skippable.*. `true` indicates skippable.
    fn with_skippable(mut self, skippable: bool) -> Self {
        self.inner.skippable = Some(skippable);
        self
    }

    /// Build the conversation turn.
    fn build(self) -> ConversationTurn {
        self.inner
    }

}

mod tests {

    use super::*;


    #[test]
    fn test_conversationturnbuilder_impl() {

        let mut ctb = ConversationTurnBuilder::new("test".to_string())
            .with_pausable(true)
            .with_skippable(true)
            .build();

        let hard_truth = ConversationTurn {
            messages:   Vec::new(),
            speaker:    String::from("test"),
            pausable:   Some(true),
            skippable:  Some(true) 
        };

        // All fields start out just as hard_truth appears.
        assert_eq!(ctb.messages, hard_truth.messages);
        assert_eq!(ctb.speaker, hard_truth.speaker);
        assert_eq!(ctb.pausable, hard_truth.pausable);
        assert_eq!(ctb.skippable, hard_truth.skippable);

    }

}
