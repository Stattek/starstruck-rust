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

    /// Create the status list for the game.
    ///
    /// # Returns
    /// - The full status list for the game
    pub fn create_status_list() -> Vec<Status> {
        vec![
            Status::new(String::from("Burn"), 10, false, 0, 5),
            Status::new(String::from("Frostburn"), 12, false, 0, 5),
        ]
    }

    /// Searches for a `Status` in a `Status` list vector.
    ///
    /// # Returns
    /// - The `Status` that was found based on the name, or `None` if no `Status` was found.
    pub fn get_status_from(target_name: &str, status_list: &Vec<Status>) -> Option<Status> {
        let mut result: Option<Status> = None;

        // go through the status list and find the one that matches our target string
        for i in 0..status_list.len() {
            if target_name == status_list[i].name() {
                result = Some(status_list[i].clone());
                break;
            }
        }

        result
    }
}
