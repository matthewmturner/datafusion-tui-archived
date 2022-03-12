use crossterm::event;
use std::time::Duration;
use std::{sync::mpsc, thread};

use crate::events::Key;

pub enum Event {
    KeyInput(Key),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: mpsc::Sender<Event>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone(); // the thread::spawn own event_tx
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);
                        event_tx.send(Event::KeyInput(key)).unwrap();
                    }
                }
                event_tx.send(Event::Tick).unwrap();
            }
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function block the current thread.
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
