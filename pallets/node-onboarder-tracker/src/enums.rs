pub enum NodeOnboardingError {
    AlreadyExists,
    InvalidFundForNode,
    NodeIdNotFound,
    
}

pub enum NodeQueryRelatedError {
    PriceCalculationError,
    InfoUpdateError,
    NodeStatusError
}