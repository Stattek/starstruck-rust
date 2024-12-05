# Starstruck
- Starstruck is a game that I first built in C++ when I was in 8th grade as part of a project for my english class (of all things).
    - It became pretty good for what I had known at the time, and I was happy with how it turned out, but it could have had more.

- This is a re-envisioning of that game but in Rust. Hopefully with some new features, as well.

## Building
Starstruck requires installing Rust to build and run, found here: <https://www.rust-lang.org/>.

Starstruck can be built and run with the following commands.

```sh
cargo build
cargo run
```

## TODO:

- Add status effects that deal damage over time (or heal over time)
    - Keep it simple for now, maybe scale it with the player's level later.
    - Added, but maybe add healing over time
- Do a uniqueness check before adding a status (so two of the same status can't exist)
    - Unless they should?
- Maybe implement some sort of rock-paper-scissors system?
    - Defending beats physical attack
    - Physical attack beats Magic attack
    - Magic attack beats defending

- Defending
    - Can currently defend without taking up a turn. This needs to be fixed, but it's kinda fun.

## TUI TODO:
- Would be awesome to have EarthBound-like backgrounds in the TUI.

