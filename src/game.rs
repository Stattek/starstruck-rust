// simple turn-based game logic

use std::io::stdin;

use rand::random;

use crate::{entities, Enemy, Entity, Player, Stats};

///the main game loop
pub fn game_loop(player: &mut Player) {
    // create a new random monster for now
    let mut monster = create_new_random_monster();

    loop {
        // each loop through here is a full turn

        // first turn
        choose_turn_from(player, &mut monster);

        // second turn
        choose_turn_from(player, &mut monster);

        if (monster.is_dead()) {
            println!("Killed the monster.");

            monster = create_new_random_monster();
        }
    }
}

///chooses to do a turn from either of these two
fn choose_turn_from(player: &mut Player, monster: &mut Enemy) {
    monster.print_info();
    if (!monster.gone_this_turn() && player.gone_this_turn()) || monster.is_faster(player) {
        // uf the monster is faster and hasn't gone yet
        monster_turn(player, monster);
    } else if !player.gone_this_turn() {
        // Otherwise, if the player hasn't gone yet
        player_turn(player, monster)
    }
}

/// does the monster's turn
fn monster_turn(player: &mut Player, monster: &mut Enemy) {
    println!("Monster did a turn");
}

/// does the player's turn
fn player_turn(player: &mut Player, monster: &mut Enemy) {
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
}

///creates a new random monster
fn create_new_random_monster() -> Enemy {
    // enemy with health between 10 and 250
    let random_health_stat: u32 = (random::<u32>() % 10) + 1;

    Enemy::new(
        String::from("test_enemy"),
        Stats::new(random_health_stat, 10, 10),
        1,
        false,
    )
}
