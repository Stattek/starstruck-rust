// simple turn-based game logic

use std::io::stdin;

use rand::random;

use crate::Move;
use crate::{Enemy, Entity, Player, Stats};

/// Enumeration to hold the type of entity something is
pub enum EntityType {
    PlayerType(Player),
    EnemyType(Enemy),
}

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
            // if we were given an enemy, use this
            the_enemy = temp_enemy;
        } else {
            // otherwise, create a new random one
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
        // create a new random monster for now
        self.enemy = create_random_monster();

        loop {
            // each loop through here is a full turn
            self.do_turns_in_order();
        }
    }

    ///Does turns in order of speed
    ///# Returns
    ///- A tuple with a `String` to represent the type of Entity that this is
    ///and a `u32` for the index into the entity `Vec`
    fn do_turns_in_order(&mut self) {
        if self.player.speed() >= self.enemy.speed() {
            // prefer player if speeds are equal
            self.do_player_turn();

            self.check_entities();
            self.enemy.get_turn_type();
        } else {
            // enemy is faster
            self.enemy.get_turn_type();
            
            self.check_entities();
            self.player.get_turn_type();
        }

        self.check_entities();
    }

    fn do_player_turn(&mut self) {
        // get the turn type
        if let Some(turn_type) = self.player.get_turn_type() {
            // now act upon this turn type
            match turn_type {
                Move::AttackMove => {
                    attack_entity(&mut self.player, &mut self.enemy);
                }
                Move::MagicMove => {}
                Move::DefendMove => {}
            }
        }
    }

    /// Checks if entities are dead and creates
    /// new random enemies if they die.
    ///
    /// If the player dies, the game is over.
    fn check_entities(&mut self) {
        if self.player.is_dead() {
            self.is_playing = false;
        } else if self.enemy.is_dead() {
            self.enemy = create_random_monster();
        }
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

/// TODO: Makes one entity attack the other.
///
/// TODO: Is it good practice to keep these out here? It is hidden from outside unless I make it public.
/// Maybe I should just do this and program it like it's C instead of C++. This is a lot easier and makes more sense to me now.
///
/// # Params
/// - `from_entity` - The entity to do the attack
/// - `victim_entity` - The entity to be attacked
fn attack_entity(from_entity: &mut dyn Entity, victim_entity: &mut dyn Entity) {
    // TODO: find out from_entity's strength and scale
    let random_attack_dmg = from_entity.get_random_attack_dmg();

    victim_entity.take_damage(random_attack_dmg);
    println!("Did {} damage!", random_attack_dmg);
}
