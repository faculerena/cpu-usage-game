use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU16, Ordering},
};

use crossterm::event::{Event, KeyCode, read};

use crate::game::GameStorage;
use crate::items::Item;

pub struct Handlers {
    pub sigkill: Arc<AtomicBool>,
    pub keystrokes: Arc<AtomicU16>,
}

impl Handlers {
    fn new(ctrl_c: Arc<AtomicBool>, keys: Arc<AtomicU16>) -> Self {
        Self { sigkill: ctrl_c, keystrokes: keys }
    }
}

pub enum HandlerInstruction {
    Stop,
    BuyItem(Item),
    Nothing,
}

impl Handlers {
    pub fn keys_pressed_for_items(&self, game_storage: &mut GameStorage) -> HandlerInstruction {
        match self.keystrokes.load(Ordering::SeqCst) {
            0 => { HandlerInstruction::Nothing }
            113 => {
                HandlerInstruction::Stop
            }
            v => {
                game_storage.items.push(Item::from(v));
                println!("Pressed key: {}", v);
                self.keystrokes.store(0, Ordering::SeqCst);
                HandlerInstruction::BuyItem(Item::from(v))
            }
        }
    }
}

pub fn start_ctrl_c_handler() -> Arc<AtomicBool> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
        .expect("Error setting Ctrl-C handler");

    running
}

pub fn read_keys_after_enter() -> Arc<AtomicU16> {
    let key = Arc::new(AtomicU16::new(0));
    let k = key.clone();

    std::thread::spawn(move || loop {
        if let Ok(Event::Key(key)) = read()
            && let KeyCode::Char(c) = key.code {
            println!("Pressed key: {}", c);
            k.store(c as u16, Ordering::SeqCst);
        }
    });
    key
}

pub fn start_handlers() -> Handlers {
    Handlers::new(start_ctrl_c_handler(), read_keys_after_enter())
}
