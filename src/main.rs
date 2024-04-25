#![feature(let_chains)]

use std::{io::Write, thread::sleep};
use std::io;
use std::io::Stdout;
use std::sync::atomic::Ordering;

use crossterm::{ExecutableCommand, terminal};
use sysinfo::{MINIMUM_CPU_UPDATE_INTERVAL as TIME_INTERVAL, System};

use crate::game::*;
use crate::handlers::*;

mod handlers;

mod game;
mod items;

fn main() -> io::Result<()> {
    let (mut pc_status, mut stdout, mut game_storage) = start_game();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let handlers = start_handlers();

    'game_loop: while handlers.sigkill.load(Ordering::SeqCst) {
        pc_status.refresh_cpu();
        let cpu_usage = pc_status.global_cpu_info().cpu_usage();

        let (rate, roi) = &game_storage.update(cpu_usage);

        match handlers.keys_pressed_for_items(&mut game_storage) {
            HandlerInstruction::Nothing => {}
            HandlerInstruction::BuyItem(item) => {
                game_storage.buy(item)
            }
            HandlerInstruction::Stop => { break 'game_loop; }
        }

        println!(
            "{esc}[2J{esc}[1;1H\n\
            /==================================\\\n\
            | CPU Usage: {:.2}%\n\
            | Coins: {}\n\
            | Rate of Income: {} * {} = {}\n\
            | Items bought: {}\n\
            \\==================================/",
            cpu_usage,
            game_storage.coins,
            roi,
            rate,
            roi * rate,
            game_storage.items,
            esc = 27 as char
        );

        sleep(TIME_INTERVAL * game_storage.rate_of_slowdown);
        stdout.flush()?;
    }


    println!("Game Over! You have earned {} coins!", game_storage.coins);

    Ok(())
}

fn start_game() -> (System, Stdout, GameStorage) {
    let pc_status = System::new();

    let stdout = io::stdout();
    let user_args: Vec<String> = std::env::args().collect();
    let rate_of_slowdown = if user_args.len() > 1 {
        user_args[1].parse::<u32>().unwrap_or(1)
    } else {
        1
    };
    let game_storage = GameStorage::new(rate_of_slowdown);

    (pc_status, stdout, game_storage)
}
