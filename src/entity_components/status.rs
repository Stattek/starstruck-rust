// file for status effects implementation

/// Very basic status effect struct
#[derive(Clone)]
pub struct Status {
    name: String,
    base_amount: u32,
    is_healing: bool,
    magic_strength_when_applied: u32, // the magic strength of the caster when the status was applied
    num_turns: u32,                   // duration of effect
}

impl Status {
    /// Create a new status effect.
    pub fn new(
        name: String,
        base_amount: u32,
        is_healing: bool,
        magic_strength_when_applied: u32, // the magic strength of the caster when the status was applied
        num_turns: u32,
    ) -> Self {
        Self {
            name,
            base_amount,
            is_healing,
            magic_strength_when_applied,
            num_turns,
        }
    }

    /// Calculates the amount of damange/health the status does.
    ///
    /// # Params
    /// - `magic_strength` - The magic strength of the entity that caused the status
    ///
    /// # Returns
    /// - A u32 containing the amount healed/damage done.
    pub fn calculate_amount(&self) -> u32 {
        // TODO: balance
        self.magic_strength_when_applied
            + self.base_amount
            + (rand::random::<u32>() % (self.magic_strength_when_applied + self.base_amount / 2))
    }

    /// Ticks the status effect, marking the end of the
    /// turn for this status effect.
    ///
    /// # Returns
    /// - `true` if the status has no turns left, `false` otherwise
    pub fn tick(&mut self) -> bool {
        self.num_turns -= 1;

        self.num_turns == 0
    }

    /// Get if the status is healing
    ///
    /// # Returns
    /// - `true` if the status heals, `false` otherwise
    pub fn is_healing(&self) -> bool {
        self.is_healing
    }

    /// Gets a copy of the name of this status
    ///
    /// # Returns
    /// - A `clone` of the name of this status
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn status_chance() -> f64 {
        0.2
    }
}
