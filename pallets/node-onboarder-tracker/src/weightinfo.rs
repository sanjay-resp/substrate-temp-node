

use frame_support::weights::Weight;

pub trait WeightInfo {
	fn register_node() -> Weight;
    fn score_update() -> Weight;
}
