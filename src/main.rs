#![feature(let_chains)]

use core::sync::atomic::Ordering;
use std::{
    io::{self, Write},
    thread::sleep,
};

use crossterm::{terminal, ExecutableCommand};
use num_bigint::BigUint;
use sysinfo::MINIMUM_CPU_UPDATE_INTERVAL as TIME_INTERVAL;

use crate::{
    game::GameStorage,
    handlers::{start_handlers, HandlerInstruction},
    start_end::{post_game, start_game},
};

mod handlers;

mod game;
mod items;
mod start_end;

fn main() -> io::Result<()> {
    let (mut pc_status, mut stdout, mut game_storage, filename) = match start_game() {
        Ok((sys, out, store, filename)) => (sys, out, store, filename),
        Err(err) => {
            return Err(err);
        }
    };

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let handlers = start_handlers();

    'game_loop: while handlers.sigkill.load(Ordering::SeqCst) {
        pc_status.refresh_cpu();
        let cpu_usage = pc_status.global_cpu_info().cpu_usage();

        let (rate, roi) = &game_storage.update(cpu_usage);

        match handlers.keys_pressed_for_items() {
            HandlerInstruction::Nothing => {}
            HandlerInstruction::BuyItem(item) => {
                match game_storage.buy(&item) {
                    Ok(()) => {
                        game_storage.last_bought =
                            Some(format!("Bought item: {}", &item.description().name));
                    }
                    Err(v) => {
                        game_storage.last_bought = Some(format!(
                            "Error buying item: {}. Error: {}.",
                            &item.description().name,
                            v
                        ));
                    }
                }
                sleep(TIME_INTERVAL);
            }
            HandlerInstruction::Stop => {
                break 'game_loop;
            }
        }

        print_to_screen(&game_storage, cpu_usage, rate, roi);

        sleep(TIME_INTERVAL * game_storage.rate_of_slowdown);
        stdout.flush()?;
    }

    post_game(&game_storage, filename)
}

fn print_to_screen(game_storage: &GameStorage, cpu_usage: f32, rate: &BigUint, roi: &BigUint) {
    println!(
        "{esc}[2J{esc}[1;1H\n\
            /==================================\\\n\
            | CPU Usage: {:.2}%\n\
            | Coins: {}\n\
            | Rate of Income: {} * {} = {}\n\
            | Items bought: {}\n\
            \\==================================/\n\
            Press `<key> + Enter` to buy an item.\n\
            Press `q+enter` or Ctrl+C to exit game.\n\
            Last Action >> {}",
        cpu_usage,
        game_storage.coins,
        roi,
        rate,
        roi * rate,
        game_storage.items,
        game_storage.last_bought.clone().unwrap_or_default(),
        esc = 27 as char
    );
    //sleep(time::Duration::from_millis(1000 * time_sleep_if_bought))
}
