use enigo::Button as EnigoMouseButton;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SerializableMouseButton {
    Left,
    Middle,
    Right,
}

impl From<SerializableMouseButton> for EnigoMouseButton {
    fn from(button: SerializableMouseButton) -> Self {
        match button {
            SerializableMouseButton::Left => EnigoMouseButton::Left,
            SerializableMouseButton::Middle => EnigoMouseButton::Middle,
            SerializableMouseButton::Right => EnigoMouseButton::Right,
        }
    }
}
