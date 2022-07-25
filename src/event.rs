use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::keys::Key;

/// The kind of Event that's been triggered
pub enum EventType {
    // Direct Wrapping of Platform Events
    WinShow,
    WinClose,
    WinResize,
    KeyDown,
    KeyUp,
    MouseMove,
    MouseWheel,
    MouseMidBtnUp,
    MouseMidBtnDown,
    MouseLeftBtnUp,
    MouseLeftBtnDown,
    MouseRightBtnUp,
    MouseRightBtnDown,

    // rovella Events

    // other
    None,
}

/// allowing the displaying of EventTypes
impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::WinClose => write!(f, "WinClose"),
            EventType::WinResize => write!(f, "WinResize"),
            EventType::KeyDown => write!(f, "KeyDown"),
            EventType::KeyUp => write!(f, "KeyUp"),
            EventType::MouseMove => write!(f, "MouseMove"),
            EventType::None => write!(f, "None"),
            EventType::MouseWheel => write!(f, "MouseWheel"),
            EventType::MouseMidBtnUp => write!(f, "MouseMidBtnUp"),
            EventType::MouseMidBtnDown => write!(f, "MouseMidBtnDown"),
            EventType::MouseLeftBtnUp => write!(f, "MouseLeftBtnUp"),
            EventType::MouseLeftBtnDown => write!(f, "MouseLeftBtnDown"),
            EventType::MouseRightBtnUp => write!(f, "MouseRightBtnUp"),
            EventType::MouseRightBtnDown => write!(f, "MouseRightBtnDown"),
            _ => write!(f, "Unknown"),
        }
        .unwrap();

        Ok(())
    }
}

/// For storing signed and unsigned types
pub union EventData {
    pub m_i32: i32,
    pub m_u32: u32,
    pub m_arr2_i16: [i16;2],
    pub m_arr2_u16: [u16;2]
}

/// setting a default value
impl Default for EventData {
    fn default() -> Self {
        EventData { m_u32: 0 }
    }
}


/// The Event struct, it's packed to optimise memory access
#[repr(packed)]
pub struct Event {
    pub e_type: EventType,
    pub data: EventData
}

impl Event {
    #[inline]
    pub fn get_key(&self) -> Key {
        Key::from(unsafe { self.data.m_u32 })
    }

    #[inline]
    pub fn get_xy(&self) -> (i16, i16) {
        return unsafe { (self.data.m_arr2_i16[0], self.data.m_arr2_i16[1]) }
    }
}

/// wrapping VecDeque right now, but a rovella version is planned
pub type EventDeque = VecDeque<Event>;

pub struct EventManager {
    pub que: EventDeque
}

impl EventManager {
    #[inline]
    pub fn new() -> EventManager {
        EventManager {
            que: EventDeque::new(),
        }
    }

    /// Returns the VecDeque que with events, and borrows a mutable version of it
    #[inline]
    pub fn get_event_que(&mut self) -> &mut EventDeque {
        return self.que.borrow_mut();
    }

    /// Gets the next Event in the event que
    #[inline]
    pub fn poll_events(&mut self) -> Option<Event> {
        return self.que.pop_front();
    }
}
