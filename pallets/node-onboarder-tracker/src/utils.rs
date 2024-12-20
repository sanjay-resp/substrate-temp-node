
pub fn calculate_stake_cost(size_in_gb: f32) -> f32 {
        
        
        let base_cost:f32 = 1.0; 
        let growth_rate:f32 = 1.04; 
                
        base_cost * growth_rate.powf(size_in_gb)
    
}