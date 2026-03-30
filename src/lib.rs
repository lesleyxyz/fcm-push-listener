mod mcs {
    include!(concat!(env!("OUT_DIR"), "/mcs_proto.rs"));
}

mod error;
pub mod fcm;
pub mod firebase;
mod gcm;
mod push;
mod register;

pub use error::Error;
pub use fcm::WebPushKeys;
pub use gcm::Session;
pub use push::new_heartbeat_ack;
pub use push::DataMessage;
pub use push::Message;
pub use push::MessageStream;
pub use push::MessageTag;
pub use push::NativeDataMessage;
pub use push::NativeMessage;
pub use push::NativeMessageStream;
pub use register::register;
pub use register::register_native;
pub use register::NativeRegistration;
pub use register::Registration;
