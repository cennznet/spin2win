#![no_std]

/// The CENNZnet SDK
use contract_sdk::{prelude::*, types::{AccountId, Balance}, util};
use ink_core::{
    env::{now, Moment},
    storage,
};
use ink_lang::contract;

contract! {

    // An event indicating a player won a prize payment
    event Win {
        player: AccountId,
        // The value of the prize winnings
        value: Balance,
    }

    // An event indicating a player was rate limited
    event RateLimited {
        player: AccountId,
        // unix timestamp of when the rate limit is due to expire
        expiry: Moment,
    }

    /// A contract that transfers a random amount of asset to the given player account
    struct Spin2Win {
        rate_limiter: storage::HashMap<AccountId, Moment>,
    }

    impl Spin2Win {
        pub(external) fn spin(&mut self, player: AccountId) {

            // Check if user has called too recently
            if self.rate_limiter.contains_key(&player) {
                let expiry = self.rate_limiter[&player];
                if now() < expiry {
                    env.emit(RateLimited {
                        player: player,
                        expiry: expiry,
                    });
                    return;
                }
            }
            self.rate_limiter.insert(player, now());

            let prize = util::random_in_range(1_000, 100_000);

            Runtime::call(
                player,
                0, // gas allocation, `0` means use current meter value
                prize.into(),
                &vec![], // Empty input payload
            );

            env.emit(Win {
                    player: player,
                    value: prize.into(),
            });

        }
    }

    impl Deploy for Spin2Win {
        fn deploy(&mut self) {}
    }

}
