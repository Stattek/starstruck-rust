///Represents the type of move that an entity is making
pub enum MoveType {
    AttackMove,
    MagicMove,
    DefendMove,
    NumMoveTypes, //THIS SHOULD BE THE LAST VALUE IN THIS ENUM
}

///trait for entities
pub trait Entity {
    ///Prints the entity's name
    fn print_name(&self);

    ///Entity takes damage
    fn take_damage(&mut self, amount: u32);

    ///Entity heals
    fn heal(&mut self, amount: u32);

    ///Uses mana
    fn use_mana(&mut self, amount: u32);

    ///Get the speed of the entity
    fn speed(&self) -> u32;

    fn name(&self) -> String;

    ///Checks to see if this entity is dead
    fn is_dead(&self) -> bool;

    ///Checks to see if the entity has gone this turn
    fn gone_this_turn(&self) -> bool;

    ///Makes this Entity do its turn and make a choice
    fn get_turn_type(&mut self) -> Option<MoveType>;

    ///Generate random attack damage from the Entity's stats
    fn get_random_attack_dmg(&self) -> u32;

    ///Print the Entity info
    fn print_info(&self);
}
