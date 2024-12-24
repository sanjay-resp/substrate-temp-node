pub fn calculate_stake_cost(size_in_gb: u64) -> f32 {
        let base_cost: f32 = 1.0;
        let growth_rate: f32 = 1.04;
    
        base_cost * integer_pow(growth_rate, size_in_gb)
    }
    
    pub fn integer_pow(base: f32, exp: u64) -> f32 {
        (0..exp).fold(1.0, |acc, _| acc * base)
    }
    