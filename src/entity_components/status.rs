// file for status effects implementation

/// Very basic status effect struct
pub struct Status {
    base_amount: u32,
    is_healing: bool,
    magic_strength_when_applied: u32, // the magic strength of the caster when the status was applied
}

impl Status {
    /// Calculates the amount of damange/health the status does.
    ///
    /// # Params
    /// - `magic_strength` - The magic strength of the entity that caused the status
    ///
    /// # Returns
    /// - A u32 containing the amount healed/damage done.
    pub fn calculate_amount(&self, magic_strength: u32) -> u32 {
        // TODO: balance
        magic_strength
            + self.base_amount
            + (rand::random::<u32>() % (magic_strength + self.base_amount / 2))
    }
}
