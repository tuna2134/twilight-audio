//! Mappings of objects received from Discord's voice gateway API, with implementations for
//! (de)serialisation.
#![deny(rustdoc::broken_intra_doc_links)]

mod close_code;
pub mod constants;
mod event;
pub mod payload;
mod protocol_data;
mod speaking_state;
mod util;

pub use twilight_model::voice::OpCode;

pub use self::{
    close_code::CloseCode, event::Event, protocol_data::ProtocolData, speaking_state::SpeakingState,
};
