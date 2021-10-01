//! The Events module. Events in Cheetah are currently blocking, meaning they are dispatched and dealt
//! with immediately. It could be that in the future a buffered event queue could be implemented, but
//! this is simpler for now.

pub mod key_event;

use std::fmt::{self, Display};

use super::bitmap::Bitmap;

pub enum EventType {
    None,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

/// Used for broad event type filtering
pub enum EventCategory {
    NoCategory,
    Application,
    Input,
    Keyboard,
    Mouse,
    MouseButton,
}

impl EventCategory {
    fn flag(&self) -> Bitmap {
        match self {
            EventCategory::NoCategory => Bitmap::from(0),
            EventCategory::Application => Bitmap::from_set(0).unwrap(),
            EventCategory::Input => Bitmap::from_set(1).unwrap(),
            EventCategory::Keyboard => Bitmap::from_set(2).unwrap(),
            EventCategory::Mouse => Bitmap::from_set(3).unwrap(),
            EventCategory::MouseButton => Bitmap::from_set(4).unwrap(),
        }
    }
}

pub trait EventBehaviour {
    fn get_event_type(&self) -> EventType;
    fn get_name(&self) -> &str;
    fn get_category_flags(&self) -> Bitmap;
    fn is_handled(&self) -> bool;
    fn is_in_category(&self, category: EventCategory) -> bool {
        (self.get_category_flags() & category.flag()).to_u64() > 0
    }
}

impl Display for dyn EventBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
