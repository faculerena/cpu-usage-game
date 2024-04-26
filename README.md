# The CPU usage game

This "game" was born when `rustc` made my pc so slow that
even [Cookie Clicker](https://orteil.dashnet.org/cookieclicker/) was slow.

## The idea

If sometimes the CPU is at >99% and the games became slow, what is the impediment to make a game that when your CPU is
being _really_ used, it goes FASTER? This is just
a [Paperclip Maximizer](https://www.lesswrong.com/tag/squiggle-maximizer-formerly-paperclip-maximizer) or Cookie
Clicker (i.e., an idle game) that goes faster (linearly) with your CPU usage.

## How to run

Clone the repo and go inside, then

```shell
cargo install --path . 
```

Then you can run this as

```shell
cpu-usage-game
```

## Instructions

When you start the game, you can buy items pressing a key and then pressing Enter.

If you press `q` + `Enter` or `Ctrl + c`, the game saves and exits.

The save file is located in the same directory you run it.

There are a few flags:

`--rate_of_slowdown` (reduce the cicle frequency by a factor)

`--new_game` (ignores the save in folder)

`--save_name` (reads and saves to a specific file instead of writing to `save.json`)

## TODO list

- Make "discounts" or something based on CPU temps.
- Item price should grow with each previous purchase (maybe an exponential growth, maybe factorial, I don't really know)