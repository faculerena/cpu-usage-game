use std::{
    fs::File,
    io::{self, Stdout},
};

use clap::Parser;
use sysinfo::System;

use crate::game::GameStorage;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'r', long, default_value_t = 1)]
    rate_of_slowdown: u32,

    #[arg(short = 'n', long, default_value_t = true)]
    new_game: bool,

    #[arg(short = 's', long)]
    save_name: Option<String>,
}

pub fn start_game() -> io::Result<(System, Stdout, GameStorage, String)> {
    let pc_status = System::new();
    let stdout = io::stdout();
    let user_args: Args = Args::parse();

    let mut filename = user_args.save_name.unwrap_or(String::from("save.json"));
    if !filename.ends_with(".json") {
        filename.push_str(".json")
    }

    let game_storage = if user_args.new_game {
        GameStorage::new(user_args.rate_of_slowdown)
    } else {
        {
            let file = match File::open(&filename) {
                Ok(reader) => reader,
                Err(err) => {
                    return Err(err);
                }
            };
            serde_json::from_reader(file)?
        }
    };

    Ok((pc_status, stdout, game_storage, filename))
}

pub fn post_game(game_storage: &GameStorage, filename: String) -> io::Result<()> {
    println!("Game Over! You have earned {} coins!", game_storage.coins);
    game_storage.save(&filename)
}
