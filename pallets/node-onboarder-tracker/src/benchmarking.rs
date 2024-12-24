#![cfg(feature = "runtime-benchmarks")]
pub mod benchmarks {
    use super::*;
    use frame_benchmarking::{benchmarks, whitelisted_caller};
    use frame_system::Pallet as System;
    use pallet_balances::Pallet as Balances;
    use frame_support::assert_ok;

    #[benchmark]
    fn register_node() {
        // Setup: Initializing the parameters for the test
        let caller: T::AccountId = whitelisted_caller();
        
        // Creating an example node info
        let node_infos = NodeInfo {
            node_id: [0u8; 32],  // Example node ID
            capacity: 100,        // Example node capacity
        };

        // Calculate the stake amount necessary for the node registration
        let needed_amount = calculate_stake_cost(node_infos.capacity as f32);
        let stake_amount = (needed_amount * 1_000_000_000_000.0) as u128; // Assuming 18 decimal places
        let stake_amount = T::Balance::from(stake_amount);

        // Reserve the stake amount (simulating the balance check)
        T::Currency::reserve(&caller, stake_amount).unwrap();

        // Perform the extrinsic call: register node
        #[extrinsic_call]
        Pallet::<T>::register_node_info(RawOrigin::Signed(caller.clone()), node_infos);

        // Verify that the event has been emitted
        let events = System::<T>::events();
        let expected_event = Event::<T>::NewNodeRegistered { node_id: node_infos.node_id };
        
        // Check if the event is in the emitted events
        assert!(events.iter().any(|record| record.event == expected_event));

        // Verification: Check if the node registration was successful
        let user_nodes = NodesOnboarded::<T>::get(&caller);
        assert_eq!(user_nodes.len(), 1);
        assert_eq!(user_nodes[0].node_id, node_infos.node_id);
    }
}
