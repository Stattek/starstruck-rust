use std::io::stdin;

use rand::random;

use crate::{entities, Enemy, Entity, Player};

pub fn game_loop(player: &Player) {
    // create the monster
    let mut monster = create_new_monster();

    loop {
        // TODO: choose which entity to go first
        monster.print_info();

        println!("Select a choice.\n1. Attack");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Invalid choice");

        if let Ok(input_value) = input.trim().parse::<u32>() {
            match input_value {
                1 => {
                    monster.take_damage(20);
                }
                _ => (),
            }
        }

        if (monster.is_dead()) {
            println!("Killed the monster.");

            monster = create_new_monster();
        }
    }
}

fn create_new_monster() -> Enemy {
    // enemy with health between 10 and 250
    let random_health: u32 = (random::<u32>() % 240) + 11;

    Enemy::new(String::from("test_enemy"), random_health, 20, 5, 1)
}
