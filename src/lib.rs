#![no_std]

/// The CENNZnet SDK
use contract_sdk::{prelude::*, util};
use ink_core::env::DefaultSrmlTypes;
use ink_lang::contract;

contract! {
    #![env = DefaultSrmlTypes]
    /// A contract that transfers a random amount of asset to the given player account
    struct Spin2Win {}

    impl Spin2Win {
        pub(external) fn spin(&self, player: AccountId) {
            let prize = util::random_in_range(1, 100);
            Runtime::call(
                player,
                0, // gas allocation, `0` means use current meter value
                prize.into(),
                &vec![], // Empty input payload
            );
        }
    }

    impl Deploy for Spin2Win {
        fn deploy(&mut self) {}
    }

}
