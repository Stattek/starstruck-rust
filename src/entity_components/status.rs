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
}
