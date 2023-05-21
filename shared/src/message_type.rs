#[repr(u8)]
pub enum MessageType {
    Setup,
    Authentification,
}

#[derive(Debug, Default, thiserror::Error)]
#[error("fail to parse MessageType")]
pub struct MessageTypeError {}

impl TryFrom<u8> for MessageType {
    type Error = MessageTypeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(MessageType::Setup),
            1 => Ok(MessageType::Authentification),
            _ => Err(MessageTypeError::default()),
        }
    }
}
