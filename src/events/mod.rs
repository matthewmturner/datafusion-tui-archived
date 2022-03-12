pub mod events;
mod key;

use crossterm::event::KeyEvent;

pub enum InputEvent {
    /// Key event occured.
    Input(KeyEvent),
    /// Tick event occured
    Tick,
}

pub use {self::key::Key, events::Event, events::Events};
