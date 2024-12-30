
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
