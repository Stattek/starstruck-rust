# Starstruck
- Starstruck is a game that I first built in C++ when I was in 8th grade as part of a project for my english class (of all things).
    - It became pretty good for what I had known at the time, and I was happy with how it turned out, but it could have had more.

- This is a re-envisioning of that game but in Rust. Hopefully with some new features, as well.

## Future
- Want to get a terminal UI working. I want to be able to play the game through a UI instead of having to type choices in.
    - Or—at the very least—I wish to make the game run as if it is a single window with text changing instead of it just printing out so much text that the previous text goes off screen. This would be preferrable.
- Possibly use something like Ratatui: <https://github.com/ratatui-org/ratatui>

## TODO:

- Add enemy random generation, using a vector of `Enemy` objects as the basis for creating the enemy
    - Scale with the player (or create an instance of an enemy that is high enough level for the player to fight)?
        - I think I'm leaning toward having the player progressively fight more and more enemies that make less and less sense maybe
- Add status effects that deal damage over time (or heal over time)
    - Keep it simple for now, maybe scale it with the player's level later.
    - Added, but maybe add healing over time
- Do a uniqueness check before adding a status (so two of the same status can't exist)
    - Unless they should?
- Maybe implement some sort of rock-paper-scissors system?
    - Defending beats physical attack
    - Physical attack beats Magic attack
    - Magic attack beats defending

## TUI TODO:
- Would be awesome to have EarthBound-like backgrounds in the TUI.