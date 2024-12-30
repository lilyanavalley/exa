
// This file is part of EXA.
// EXA is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as 
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// EXA is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with EXA. If not, see 
// <https://www.gnu.org/licenses/>.

use std::{ error::Error, fmt::Display };
use fluent_bundle::FluentError;
use fluent_syntax::parser::ParserError;


#[derive(Debug)]
pub enum DialogError {

    /// Conversation does not exist.
    ConversationNonexistent,

    /// Speaker does not exist.
    SpeakerNonexistent,

    /// Ticket is pointing to a non-existent conversation.
    TicketInvalid,

    /// Ticket is pointing to a non-existent turn in a conversation.
    TicketTurnInvalid,

    /// Markdown parse error in FTL message.
    ParseErrorMarkdown(()),

    /// Fluent parse error.
    ParseErrorFtl(FluentError),

}

impl Display for DialogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DialogError {}

mod tests {

    use super::*;
    use fluent_bundle::FluentError;
    // use fluent_syntax::parser::ParserError;


    #[test]
    fn test_dialogerror_display() {

        let error = DialogError::ConversationNonexistent;
        assert_eq!(error.to_string(), "ConversationNonexistent");

        let error = DialogError::SpeakerNonexistent;
        assert_eq!(error.to_string(), "SpeakerNonexistent");
        
        let error = DialogError::TicketInvalid;
        assert_eq!(error.to_string(), "TicketInvalid");
        
        let error = DialogError::TicketTurnInvalid;
        assert_eq!(error.to_string(), "TicketTurnInvalid");
        
        let error = DialogError::ParseErrorMarkdown(());
        assert_eq!(error.to_string(), "ParseErrorMarkdown(())");
        
        let error_ftl = ParserError {
            pos: 0..1,
            slice: Some(0..1),
            kind: fluent_syntax::parser::ErrorKind::ExpectedLiteral
        };
        let error = DialogError::ParseErrorFtl(FluentError::ParserError(error_ftl.clone()));
        assert_eq!(error.to_string(), format!("ParseErrorFtl(ParserError({:?}))", error_ftl));
        // TODO: Consider changing these last few lines to better test type wrapping ^^^^^^^^
        

    }

}
