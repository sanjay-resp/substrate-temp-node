use crate::enums::NodeQueryRelatedError;

pub trait NodeOnChainInfo<AccountId>{
    fn get_price_quote(owner: &AccountId, item_type: &[u8], item: &[u8]) -> Result<(),NodeQueryRelatedError>;
    fn get_total_network_storage() -> Result<(),NodeQueryRelatedError>;
    fn get_up_nodes() -> Result<(),NodeQueryRelatedError>;
    fn get_down_nodes()-> Result<(),NodeQueryRelatedError>;
}