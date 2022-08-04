use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::keys::Key;

/// The kind of Event that's been triggered
pub enum Event {
    // Direct Wrapping of Platform Events
    WinShow,
    WinClose,
    WinResize,
    KeyDown(Key),
    KeyUp(Key),
    MouseMove(i16, i16),
    MouseWheel(i32),
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
impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::WinClose => write!(f, "WinClose"),
            Event::WinResize => write!(f, "WinResize"),
            Event::KeyDown(_) => write!(f, "KeyDown"),
            Event::KeyUp(_) => write!(f, "KeyUp"),
            Event::MouseMove(_,_) => write!(f, "MouseMove"),
            Event::None => write!(f, "None"),
            Event::MouseWheel(_) => write!(f, "MouseWheel"),
            Event::MouseMidBtnUp => write!(f, "MouseMidBtnUp"),
            Event::MouseMidBtnDown => write!(f, "MouseMidBtnDown"),
            Event::MouseLeftBtnUp => write!(f, "MouseLeftBtnUp"),
            Event::MouseLeftBtnDown => write!(f, "MouseLeftBtnDown"),
            Event::MouseRightBtnUp => write!(f, "MouseRightBtnUp"),
            Event::MouseRightBtnDown => write!(f, "MouseRightBtnDown"),
            _ => write!(f, "Unknown"),
        }
        .unwrap();

        Ok(())
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
