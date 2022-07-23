use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::keys::Key;

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

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::WinClose => write!(f, "WinClose"),
            EventType::WinResize => write!(f, "WinResize"),
            EventType::KeyDown => write!(f, "KeyDown"),
            EventType::KeyUp => write!(f, "KeyUp"),
            EventType::MouseMove => write!(f, "MouseMove"),
            EventType::None => write!(f, "None"),
            _ => write!(f, "Unknown"),
        }
        .unwrap();

        Ok(())
    }
}

pub union EventData {
    pub signed: i32,
    pub unsigned: u32,
}

impl Default for EventData {
    fn default() -> Self {
        EventData { unsigned: 0 }
    }
}

#[repr(packed)]
pub struct Event {
    pub e_type: EventType,
    pub data0: EventData,
    pub data1: EventData,
}

impl Event {
    #[inline]
    pub fn get_key(&self) -> Key {
        Key::from(unsafe { self.data0.unsigned })
    }

    #[inline]
    pub fn get_xy(&self) -> (i32, i32) {
        return unsafe { (self.data0.signed, self.data1.signed) }
    }
}

pub type EventDeque = VecDeque<Event>;

pub struct EventManager {
    pub que: EventDeque,
}

impl EventManager {
    #[inline]
    pub fn new() -> EventManager {
        EventManager {
            que: EventDeque::new(),
        }
    }

    #[inline]
    pub fn get_event_que(&mut self) -> &mut EventDeque {
        return self.que.borrow_mut();
    }

    #[inline]
    pub fn poll_events(&mut self) -> Option<Event> {
        return self.que.pop_front();
    }
}
