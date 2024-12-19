use std::collections::VecDeque;

use rand::random;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::entity_components::{entity::Entity, moves::Move, moves::MoveType, stats::Stats};

use super::status::Status;

/// Struct to represent an enemy.
///
/// Implements the Entity trait.
#[derive(Clone)]
pub struct Enemy {
    name: String,
    health: u32,
    max_health: u32,
    mana: u32,
    max_mana: u32,
    stats: Stats,
    level: u32,
    has_gone: bool,
    statuses: Vec<Status>,
    move_set: Vec<MoveType>,
}

const NUM_MOVES_TO_ADD: u32 = 4;
const BASE_XP: u32 = 20; // the base xp dropped by an enemy

impl Enemy {
    /// Create a new `Enemy`
    ///
    /// # Params
    /// - `name` - The name of the `Enemy`.
    /// - `stats` - The `Stats` of the `Enemy`.
    /// - `level` - The level of the `Enemy`
    /// - `has_gone` - If this `Enemy` has gone this turn.
    pub fn new(name: String, stats: Stats, level: u32, has_gone: bool) -> Self {
        let starting_health = stats.calculate_max_health();
        let starting_mana = stats.calculate_max_mana();
        Self {
            name,
            health: starting_health,
            max_health: starting_health,
            mana: starting_mana,
            max_mana: starting_mana,
            stats,
            level,
            has_gone,
            statuses: Vec::new(), // start with no statuses,
            move_set: Enemy::get_rand_move_set(NUM_MOVES_TO_ADD), // initialize with a random set of moves
        }
    }

    /// Calculate the xp dropped by this `Enemy`.
    ///
    /// # Params
    /// - `player_level` - The level of the player.
    ///
    /// # Returns
    /// - The xp dropped by this `Enemy`.
    pub fn drop_xp(&self, player_level: u32, text_vec: &mut VecDeque<String>) -> u32 {
        let mut amount = BASE_XP; // start with a base xp

        let num_levels_above_player = self.level as i64 - player_level as i64;
        for _i in 0..num_levels_above_player {
            amount *= 2; // just crazy xp as enemies get way higher leveled than you
        }

        text_vec.push_back(format!("{} dropped {} xp!", self.name, amount));

        amount
    }

    /// Display the text for attacking another `Entity`.
    ///
    /// # Params
    /// - `victim_entity_name` - The name of the `Entity` that is receiving the attack.
    /// - `damage_dealt` - The amount of damage dealt to this `Entity`.
    fn display_attack_text(
        &self,
        victim_entity_name: &String,
        damage_dealt: u32,
        damage_type: String,
        text_vec: &mut VecDeque<String>,
    ) {
        text_vec.push_back(format!(
            "{} did {} {} damage to {}",
            self.name,
            damage_dealt.to_string(),
            damage_type,
            victim_entity_name
        ));
    }

    /// Create the enemy list for the game
    ///
    /// # Returns
    /// - The full enemy list for the game
    pub fn create_enemy_list() -> Vec<Enemy> {
        vec![
            Enemy::new("Spider".to_string(), Stats::new(5, 0, 4, 2, 1, 0), 1, false),
            Enemy::new(
                "Skeleton".to_string(),
                Stats::new(3, 0, 3, 5, 4, 0),
                1,
                false,
            ),
            Enemy::new(
                "Dragon".to_string(),
                Stats::new(20, 100, 10, 10, 10, 10),
                8,
                false,
            ),
        ]
    }

    /// The `Enemy` performs a magic attack move against another `Entity`.
    ///
    /// # Params
    /// - `target` - The target of the attack.
    /// - `text_vec` - The text `VecDeque` for displaying attack text.
    ///
    /// # Returns
    /// - `true` on success, `false` otherwise.
    pub fn magic_move(&mut self, target: &mut dyn Entity, text_vec: &mut VecDeque<String>) -> bool {
        // check whether the user has already gone or if the move costs too much
        if self.has_gone {
            return false; // error
        }

        // TODO: make a new function to generate this?
        // TODO: balance?
        let random_damage = self.magic_strength()
            + (rand::random::<u32>() % (self.magic_strength() + self.magic_strength() / 2));

        let damage_dealt = self.attack_entity(random_damage, target);

        // display the damage that was dealt
        self.display_attack_text(
            target.name(),
            damage_dealt,
            Move::damage_type(MoveType::MagicMove),
            text_vec,
        );

        // the enemy has gone
        self.has_gone = true;

        // no error
        true
    }

    /// Generates the list of next moves for the `Enemy` to perform. Returns a `Vec`
    /// of these moves.
    ///
    /// # Params
    /// - `num_moves` - The number of moves to generate.
    ///
    /// # Returns
    /// - A `Vec` of all the next moves for the `Enemy` to perform.
    fn get_rand_move_set(num_moves: u32) -> Vec<MoveType> {
        let mut output = vec![];
        for _ in 0..num_moves {
            let rand_num = random::<u32>() % MoveType::NumMoveTypes as u32;

            // doesn't really matter what order these are in or numbers used, just that they are here
            let move_type = match rand_num {
                0 => MoveType::AttackMove,
                1 => MoveType::MagicMove,
                2 => MoveType::DefendMove,
                _ => {
                    panic!("Invalid move type generated by Enemy")
                }
            };

            output.push(move_type);
        }

        output
    }

    /// Gets a `ListItem` from all of the possible moves that this `Enemy` can perform.
    /// NOTE: For use with a Ratatui UI function.
    ///
    /// # Returns
    /// - The `ListItem`representing the moves that the `Enemy` can perform.
    pub fn get_move_set_listitem(&self) -> ListItem {
        let mut move_spans = vec![];
        move_spans.push(Span::styled(
            "    Move Set: ",
            Style::default().fg(Color::Cyan),
        ));

        let move_set_iter = self.move_set.iter().rev();
        for move_type in move_set_iter {
            let temp = match move_type {
                MoveType::AttackMove => {
                    Span::styled("A", Style::default().bg(Color::Red).fg(Color::White))
                }
                MoveType::MagicMove => {
                    Span::styled("M", Style::default().bg(Color::Blue).fg(Color::White))
                }
                MoveType::DefendMove => {
                    Span::styled("D", Style::default().bg(Color::White).fg(Color::Black))
                }
                MoveType::NumMoveTypes => {
                    panic!("Cannot get next move for listitem")
                }
            };

            move_spans.push(temp);
        }
        // combine all of the spans into a line
        let line = Line::from(move_spans);

        // then return the final ListItem
        ListItem::from(line)
    }

    /// The Enemy makes a choice as to what type of move it wants to do this turn.
    /// Chooses from its current `move_set`.
    ///
    /// # Returns
    /// - `Option<MoveType>` with `Some(MoveType)` on success and `None` on failure.
    pub fn get_turn_type(&mut self) -> Option<MoveType> {
        if self.move_set.is_empty() {
            // generate a new list of moves
            self.move_set = Enemy::get_rand_move_set(NUM_MOVES_TO_ADD);
        }
        let output = self.move_set.pop();

        if self.move_set.is_empty() {
            // generate a new list of moves if popping this emptied the vec
            self.move_set = Enemy::get_rand_move_set(NUM_MOVES_TO_ADD);
        }

        output
    }
}

impl Entity for Enemy {
    fn take_damage(&mut self, amount: u32) -> u32 {
        let damage_taken = self.stats.calc_damage_taken(amount);

        if damage_taken > self.health {
            self.health = 0;
        } else {
            self.health -= damage_taken;
        }

        damage_taken
    }

    fn heal(&mut self, amount: u32) {
        self.health += amount;
    }

    fn use_mana(&mut self, amount: u32) {
        if amount > self.mana {
            self.mana = 0;
        } else {
            self.mana -= amount;
        }
    }

    fn speed(&self) -> u32 {
        self.stats.get_speed()
    }

    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn get_random_attack_dmg(&self) -> u32 {
        self.stats.generate_random_attack_dmg()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn level(&self) -> u32 {
        self.level
    }

    fn magic_strength(&self) -> u32 {
        self.stats.get_magic_strength()
    }

    fn start_defending(&mut self) {
        self.stats.start_defending();
    }

    fn stop_defending(&mut self) {
        self.stats.stop_defending()
    }

    fn tick_statuses(&mut self, text_vec: &mut VecDeque<String>) {
        let mut indicies_to_remove: Vec<usize> = Vec::new();

        for i in 0..self.statuses.len() {
            // FIXME: there is a bug with this going out of bounds
            let amount = self.statuses[i].calculate_amount();

            // mark this status for removal if it has no turns left
            if self.statuses[i].tick() {
                // push this index
                indicies_to_remove.push(i);
            }

            if self.statuses[i].is_healing() {
                text_vec.push_back(format!(
                    "{} healed {} health from {}!",
                    self.name,
                    amount.to_string().as_str(),
                    self.statuses[i].name()
                ));
                self.heal(amount);
            } else {
                text_vec.push_back(format!(
                    "{} took {} damage from {}!",
                    self.name,
                    amount.to_string().as_str(),
                    self.statuses[i].name()
                ));
                self.take_damage(amount);
            }

            let mut cur_num_removed = 0;
            // remove all statuses that were marked for removal
            for i in 0..indicies_to_remove.len() {
                // since the indices of the elements will change due to the removal
                let index = indicies_to_remove[i] - cur_num_removed;
                self.statuses.remove(index);
                cur_num_removed += 1; // we have removed another status
            }
        }
    }

    fn apply_status(&mut self, status: &Status, text_vec: &mut VecDeque<String>) {
        text_vec.push_back(format!("{} applied to {}", status.name(), self.name()));
        self.statuses.push(status.clone());
    }

    fn attack_move(&mut self, target: &mut dyn Entity, text_vec: &mut VecDeque<String>) -> bool {
        if self.has_gone {
            return true; // has gone, error
        }
        // attack the player with a random amount of damage
        let random_damage = self.get_random_attack_dmg();

        let damage_dealt = self.attack_entity(random_damage, target);
        // display the text for an attack
        self.display_attack_text(
            target.name(),
            damage_dealt,
            Move::damage_type(MoveType::AttackMove),
            text_vec,
        );

        // the enemy has gone
        self.has_gone = true;

        // no error
        false
    }

    fn defend_move(&mut self, text_vec: &mut VecDeque<String>) -> bool {
        if self.has_gone {
            return false; // error
        }
        self.start_defending();

        // tell player that they started defending
        let mut output_str = String::new();
        output_str.push_str(&self.name.as_str());
        output_str.push_str(" began defending for 1 turn.");

        text_vec.push_back(format!("{} began defending for 1 turn.", self.name));

        // the enemy has gone
        self.has_gone = true;

        // no error
        true
    }

    fn has_gone(&self) -> bool {
        self.has_gone
    }

    fn health(&self) -> u32 {
        self.health
    }

    fn max_health(&self) -> u32 {
        self.max_health
    }

    fn allow_move(&mut self) {
        // we can move again
        self.has_gone = false;
    }
}
