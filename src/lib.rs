#![no_std]

use contract_sdk::{
    ink_core::{self, env::DefaultSrmlTypes},
    ink_model::{self},
    ink_lang::contract,
    prelude::*,
    util,
};

contract! {
    #![env = DefaultSrmlTypes]

    // A contract that transfers a random amount of asset to the given player account
    struct Spin2Win {}

    impl Spin2Win {
        pub(external) fn spin(&self) {
            let prize = util::random_in_range(1, 100);
            Runtime::call(
                env.caller(), // Use contract caller account is the player
                0,            // gas allocation, `0` means use current meter value
                prize.into(),
                &vec![],      // Empty input payload
            );
        }
    }

    impl Deploy for Spin2Win {
        fn deploy(&mut self) {}
    }

}
