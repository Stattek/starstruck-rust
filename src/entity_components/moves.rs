pub enum ElementType {
    Fire,
    Wind,
    Earth,
    Water,
    NumElements, // This should always be the last value
}

pub enum MoveType {
    AttackMove,
    MagicMove,
    DefendMove,
    NumMoveTypes, // This should always be the last value
}

/// Struct for representing a move in the game.
/// This could be an attacking or healing move.
///
/// # FUTURE:
/// - Create status effects like what was being worked on in the Java version.
pub struct Move<'a> {
    name: &'a str, // specify the lifetime of this variable (still don't know why)
    base_amount: u32,
    mana_cost: u32,
    level_requirement: u32,
    element: ElementType,
}

impl<'a> Move<'a> {
    pub fn new(
        name: &'a str,
        base_amount: u32,
        mana_cost: u32,
        level_requirement: u32,
        element: ElementType,
    ) -> Self {
        Self {
            name,
            base_amount,
            mana_cost,
            level_requirement,
            element,
        }
    }

    /// Generates a random damage/healing value for this `Move`.
    ///
    /// # Params
    /// - `magic_strength` - The magic strength stat of the user to enhance the Move's damage/healing.
    ///
    /// # Returns
    /// - The random value of healing/damage this `Move` will do.
    pub fn generate_random_amount(&self, magic_strength: u32) -> u32 {
        // magic_strength + (random number between 0 and magic_strength/2)
        magic_strength + (rand::random::<u32>() % (magic_strength / 2))
    }

    /// Creates and returns a `Move` list of all the moves that are in the game.
    ///
    /// # Returns
    /// - A `Vec` of all the `Move`s in the game.
    pub fn get_move_list() -> Vec<Move<'a>> {
        vec![
            Move::new("FireOne", 10, 2, 1, ElementType::Fire),
            Move::new("WindOne", 12, 2, 3, ElementType::Wind),
            Move::new("EarthOne", 14, 2, 5, ElementType::Earth),
            Move::new("WaterOne", 17, 2, 6, ElementType::Water),
        ]
    }

    /// Checks that the entity with this level meets the requirements for using
    /// this `Move`.
    ///
    /// # Params
    /// - `entity_level` - The level of the entity using this `Move`.
    ///
    /// # Returns
    /// - Returns `true` if the entity meets the requirements, `false` otherwise
    pub fn is_meeting_requirements(&self, entity_level: u32) -> bool {
        self.level_requirement <= entity_level
    }

    /// Get the mana cost of this move.
    ///
    /// # Returns
    /// - the mana cost of the `Move`.
    pub fn cost(&self) -> u32 {
        self.mana_cost
    }

    /// Get the name of this move.
    ///
    /// # Returns
    /// - the name of the `Move`.
    pub fn name(&self) -> &'a str {
        self.name
    }
}
