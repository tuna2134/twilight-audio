use std::{
    convert::TryFrom,
    fmt::{self, Display},
};

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Voice close event codes.
///
/// See [Discord Docs/Voice Close Event Codes] for more information.
///
/// [Discord Docs/Voice Close Event Codes]: https://discord.com/developers/docs/topics/opcodes-and-status-codes#voice-voice-close-event-codes
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum CloseCode {
    /// Invalid Voice OP Code.
    UnknownOpcode = 4001,
    /// Invalid identification payload sent.
    FailedToDecodePayload = 4002,
    /// A payload was sent prior to identifying.
    NotAuthenticated = 4003,
    /// The account token sent with the identify payload was incorrect.
    AuthenticationFailed = 4004,
    /// More than one identify payload was sent.
    AlreadyAuthenticated = 4005,
    /// The session is no longer valid.
    SessionNoLongerValid = 4006,
    /// A session timed out.
    SessionTimeout = 4009,
    /// The server for the last connection attempt could not be found.
    ServerNotFound = 4011,
    /// Discord did not recognise the voice protocol chosen.
    UnknownProtocol = 4012,
    /// Disconnected, either due to channel closure/removal or kicking.
    ///
    /// Should not reconnect.
    Disconnected = 4014,
    /// Connected voice server crashed.
    ///
    /// Should resume.
    VoiceServerCrash = 4015,
    /// Discord didn't recognise the encryption scheme.
    UnknownEncryptionMode = 4016,
}

impl CloseCode {
    /// Indicates whether a voice client should attempt to resume voice in response to this close
    /// code.
    ///
    /// Otherwise, the connection should be closed.
    pub const fn can_resume(&self) -> bool {
        matches!(
            self,
            CloseCode::VoiceServerCrash | CloseCode::SessionTimeout
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CloseCodeConversionError {
    code: u16,
}

impl CloseCodeConversionError {
    const fn new(code: u16) -> Self {
        Self { code }
    }

    pub const fn code(&self) -> u16 {
        self.code
    }
}

impl Display for CloseCodeConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.code, f)?;

        f.write_str(" isn't a valid close code")
    }
}

impl std::error::Error for CloseCodeConversionError {}

impl TryFrom<u16> for CloseCode {
    type Error = CloseCodeConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            4001 => Self::UnknownOpcode,
            4002 => Self::FailedToDecodePayload,
            4003 => Self::NotAuthenticated,
            4004 => Self::AuthenticationFailed,
            4005 => Self::AlreadyAuthenticated,
            4006 => Self::SessionNoLongerValid,
            4009 => Self::SessionTimeout,
            4011 => Self::ServerNotFound,
            4012 => Self::UnknownProtocol,
            4014 => Self::Disconnected,
            4015 => Self::VoiceServerCrash,
            4016 => Self::UnknownEncryptionMode,
            _ => return Err(Self::Error::new(value)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{CloseCode, CloseCodeConversionError};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug, hash::Hash};

    assert_impl_all!(
        CloseCode: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(CloseCodeConversionError: Debug, Eq, PartialEq, Send, Sync);

    const CLOSE_CODES: [(CloseCode, u16, bool); 12] = [
        (CloseCode::UnknownOpcode, 4001, false),
        (CloseCode::FailedToDecodePayload, 4002, false),
        (CloseCode::NotAuthenticated, 4003, false),
        (CloseCode::AuthenticationFailed, 4004, false),
        (CloseCode::AlreadyAuthenticated, 4005, false),
        (CloseCode::SessionNoLongerValid, 4006, false),
        (CloseCode::SessionTimeout, 4009, true),
        (CloseCode::ServerNotFound, 4011, false),
        (CloseCode::UnknownProtocol, 4012, false),
        (CloseCode::Disconnected, 4014, false),
        (CloseCode::VoiceServerCrash, 4015, true),
        (CloseCode::UnknownEncryptionMode, 4016, false),
    ];

    #[test]
    fn variants() {
        for (kind, num, can_resume) in CLOSE_CODES {
            serde_test::assert_tokens(&kind, &[Token::U16(num)]);
            assert_eq!(kind, CloseCode::try_from(num).unwrap());
            assert_eq!(num, kind as u16);
            assert!(kind.can_resume() == can_resume)
        }
    }

    #[test]
    fn try_from_error() {
        assert!(
            matches!(CloseCode::try_from(5000), Err(CloseCodeConversionError { code }) if code == 5000)
        );
    }
}
