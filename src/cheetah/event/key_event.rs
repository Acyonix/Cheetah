//! Covers keyboard events such as KeyPressedEvent and KeyReleasedEvent

use super::Bitmap;
use super::{EventBehaviour, EventCategory, EventType};

pub trait KeyEvent: EventBehaviour {
    fn get_key_code(&self) -> u64;
}

pub struct KeyPressedEvent {
    repeated: bool,
    key_code: u64,
    handled: bool,
}

impl KeyPressedEvent {
    pub fn get_repeat_count(&self) -> bool {
        self.repeated
    }
}

impl KeyEvent for KeyPressedEvent {
    fn get_key_code(&self) -> u64 {
        self.key_code
    }
}

impl EventBehaviour for KeyPressedEvent {
    fn get_event_type(&self) -> EventType {
        EventType::KeyPressed
    }

    fn get_name(&self) -> &str {
        "KeyPressedEvent"
    }

    fn get_category_flags(&self) -> Bitmap {
        EventCategory::Keyboard.flag() & EventCategory::Input.flag()
    }

    fn is_handled(&self) -> bool {
        return self.handled;
    }
}
