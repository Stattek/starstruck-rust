// simple turn-based game logic

use std::io::stdin;

use rand::random;

use crate::{entities, Enemy, Entity, Player, Stats};

/// Struct to hold the game state.
pub struct GameState {
    players: Vec<Player>,
    enemies: Vec<Enemy>,
}

impl GameState {
    /// the main game loop
    pub fn game_loop(&mut self, player: &mut Player) {
        // create a new random monster for now
        self.create_random_monsters();

        loop {
            // each loop through here is a full turn

            // do all turns for each entity
            self.do_turns();

            self.check_and_handle_dead_entities();
        }
    }

    ///chooses to do a turn from either of these two
    fn choose_turn_from_all(&mut self) {
        // hold any entity
        let mut fastest_player_index=0;
        let mut fastest_enemy_index=0;

        // get fastest player
        for i in 0..self.players.len() {
            if let Some(some_player) = fastest_player {
                if fastest_player
            } else if !player.gone_this_turn() {
                fastest_player = Some(player);
            }

            if fastest_player.is_some()
                && fastest_player.unwrap().get_speed() < player.get_speed()
                && !player.gone_this_turn()
            {
                fastest_player = Some(player);
            }
        }

        // get fastest enemy
        for enemy in &mut self.enemies {
            if fastest_enemy.is_none() && !enemy.gone_this_turn() {
                fastest_enemy = Some(enemy);
            }

            if fastest_enemy.is_some()
                && fastest_enemy.unwrap().get_speed() < enemy.get_speed()
                && !enemy.gone_this_turn()
            {
                fastest_enemy = Some(enemy);
            }
        }

        // in case of a tie, prefer the player
        // if fastest_player.is_some(){
        // }
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
}
