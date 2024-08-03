//simple turn-based game logic

use std::char::from_u32_unchecked;
use std::io::stdin;

use colored::Colorize;
use rand::random;

use crate::entity_components::enemy::Enemy;
use crate::entity_components::entity::MoveType;
use crate::{Entity, Player, Stats};

///Struct to hold the game state.
pub struct GameState {
    player: Player,
    enemy: Enemy,
    is_playing: bool,
}

impl GameState {
    ///Create new GameState object
    pub fn new(player: Player, enemy: Option<Enemy>) -> Self {
        let is_playing = !player.is_dead();
        let the_enemy;

        if let Some(temp_enemy) = enemy {
            //if we were given an enemy, use this
            the_enemy = temp_enemy;
        } else {
            //otherwise, create a new random one
            the_enemy = create_random_monster();
        }

        GameState {
            player,
            enemy: the_enemy,
            is_playing,
        }
    }

    ///the main game loop
    pub fn game_loop(&mut self) {
        //create a new random monster for now
        self.enemy = create_random_monster();

        while self.is_playing {
            //each loop through here is a full turn
            self.do_turns_in_order();
        }
    }

    ///Does turns in order of speed
    ///# Returns
    ///- A tuple with a `String` to represent the type of Entity that this is
    ///and a `u32` for the index into the entity `Vec`
    fn do_turns_in_order(&mut self) {
        if self.player.speed() >= self.enemy.speed() {
            //prefer player if speeds are equal

            // print info
            println!(); // make a new line now
            self.player.print_info();
            self.enemy.print_info();

            // do turns
            self.do_player_turn();

            // check entities before next turn is done
            if !self.check_entities() {
                self.do_enemy_turn();
            }
        } else {
            // enemy is faster

            // do enemy turn
            self.do_enemy_turn();

            // check entities before doing the next turn
            if !self.check_entities() {
                // print info
                println!(); //make new line
                self.player.print_info();
                self.enemy.print_info();

                // player turn
                self.player.get_turn_type();
            }
        }

        self.check_entities();
    }

    fn do_player_turn(&mut self) {
        //get the turn type
        if let Some(turn_type) = self.player.get_turn_type() {
            //now act upon this turn type
            match turn_type {
                MoveType::AttackMove => {
                    attack_entity(&mut self.player, &mut self.enemy);
                }

                MoveType::MagicMove => {}
                MoveType::DefendMove => {}
                _ => {} //We should never reach this
            }
        }
    }

    fn do_enemy_turn(&mut self) {
        //get the turn type
        if let Some(turn_type) = self.enemy.get_turn_type() {
            match turn_type {
                MoveType::AttackMove => {
                    attack_entity(&mut self.enemy, &mut self.player);
                }

                MoveType::MagicMove => {}
                MoveType::DefendMove => {}
                _ => {} // We should never reach this
            }
        }
    }

    ///Checks if entities are dead and creates
    ///new random enemies if they die.
    ///
    ///If the player dies, the game is over.
    /// # Returns
    /// True if an entity died, false otherwise.
    fn check_entities(&mut self) -> bool {
        let mut output = false;

        if self.player.is_dead() {
            println!("{}", "\nYou died!".red().bold());
            self.is_playing = false;

            // entity died
            output = true;
        } else if self.enemy.is_dead() {
            println!("{}", "\nThe enemy died!".green());
            self.enemy = create_random_monster();

            //entity died
            output = true;
        }

        output
    }
}

///Creates a new random monster
fn create_random_monster() -> Enemy {
    //enemy with health between 10 and 250
    let random_health_stat: u32 = (random::<u32>() % 10) + 1;

    Enemy::new(
        String::from("test_enemy"),
        Stats::new(random_health_stat, 10, 10, 10),
        1,
        false,
    )
}

///TODO: Makes one entity attack the other.
///
///TODO: Is it good practice to keep these out here? It is hidden from outside unless I make it public.
///Maybe I should just do this and program it like it's C instead of C++. This is a lot easier and makes more sense to me now.
///
///# Params
///- `from_entity` - The entity to do the attack
///- `victim_entity` - The entity to be attacked
fn attack_entity(from_entity: &mut dyn Entity, victim_entity: &mut dyn Entity) {
    //TODO: find out from_entity's strength and scale
    let random_attack_dmg = from_entity.get_random_attack_dmg();

    victim_entity.take_damage(random_attack_dmg);

    // cursed string creation to colorize this string when we print it out 💀
    let mut output_str = String::new();
    output_str.push_str(from_entity.name().as_str());
    output_str.push_str(" did ");
    output_str.push_str(random_attack_dmg.to_string().as_str());
    output_str.push_str(" damage to ");
    output_str.push_str(victim_entity.name().as_str());

    println!("{}", output_str.red());
}
