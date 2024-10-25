///Represents the type of move that an entity is making
use crate::entity_components::moves::MoveType;

use super::status::Status;

///trait for entities
pub trait Entity {
    ///Entity takes damage
    ///
    /// # Params
    /// - `amount` - The amount of damage the entity is going to take
    ///
    /// # Returns
    /// - The amount of damage the entity actually took from the attack, such as when an entity takes less damage due to defense.
    fn take_damage(&mut self, amount: u32) -> u32;

    ///Entity heals
    ///
    /// # Params
    /// - `amount` - The amount to heal.
    fn heal(&mut self, amount: u32);

    ///Uses mana
    ///
    /// # Params
    /// - `amount` - The amount to use.
    fn use_mana(&mut self, amount: u32);

    ///Get the speed of the entity
    ///
    /// # Returns
    /// - The speed of this entity.
    fn speed(&self) -> u32;

    /// Get the level of the entity.
    ///
    /// # Returns
    /// - The level of this entity.
    fn level(&self) -> u32;

    /// Get the name of the entity.
    ///
    /// # Returns
    /// - The name of this entity.
    fn name(&self) -> String;

    /// Get the magic strength of the entity.
    ///
    /// # Returns
    /// - The magic strength of this entity.
    fn magic_strength(&self) -> u32;

    /// Checks to see if this entity is dead.
    ///
    /// # Returns
    /// - `true` if the entity is dead, `false` otherwise.
    fn is_dead(&self) -> bool;

    /// Checks to see if the entity has gone this turn.
    /// For use when there are more than 2 entities in a fight.
    ///
    /// # Returns
    /// - `true` if the `Entity` has gone this turn, `false` otherwise.
    fn gone_this_turn(&self) -> bool;

    /// Makes this Entity do its turn and make a choice.
    ///
    /// # Returns
    /// - The `MoveType` selected by the `Entity`.
    fn get_turn_type(&mut self) -> Option<MoveType>;

    /// Generate random attack damage from the Entity's stats
    ///
    /// # Returns
    /// - A random attack damage number.
    fn get_random_attack_dmg(&self) -> u32;

    /// Print the Entity info
    fn print_info(&self);

    /// Makes this `Entity` attack another `Entity`.
    ///
    /// # Params
    ///
    /// - `amount` - The amount of damage the enemy will take
    /// - `entity` - The entity to take damage
    ///
    /// # Returns
    /// - The amount of damage dealt to the enemy.
    fn attack_entity(&self, amount: u32, entity: &mut dyn Entity) -> u32 {
        entity.take_damage(amount)
    }

    /// Entity starts defending.
    fn start_defending(&mut self);

    /// Entity stops defending.
    fn stop_defending(&mut self);

    /// Ticks all statuses in vector
    fn tick_statuses(&mut self);

    /// Applies a status to this Entity.
    ///
    /// # Params
    /// - `status` The `Status` to apply to this `Entity`.
    fn apply_status(&mut self, status: &Status);

    /// Entity does a physical attack against another `Entity`.
    ///
    /// # Params
    /// - `target` - The target `Entity` receiving the attack.
    ///
    /// # Returns
    /// - `true` if the move was done, `false` if the move was not done (in case the move was canceled).
    fn attack_move(&mut self, target: &mut dyn Entity, text_vec: &mut Vec<String>) -> bool;

    /// Returns if the entity has gone this turn yet.
    fn has_gone(&self) -> bool;
}
