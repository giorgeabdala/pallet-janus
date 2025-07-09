use crate as pallet_janus;

#[cfg(test)]
mod tests {
    use frame_support::__private::sp_io;
    use super::*;
    use frame_support::{derive_impl, runtime};
    use frame_system::Config;
    use frame_system::pallet_prelude::RuntimeCallFor;
    use sp_runtime::app_crypto::sp_core;
    use sp_runtime::BuildStorage;
    use sp_core::H256;
    use sp_runtime::traits::BlakeTwo256;
    use runtime::{RuntimeEvent, RuntimeOrigin, Test, System, PalletInfo, RuntimeCall};
    
    #[runtime]
    mod runtime {
        #[runtime::runtime]
        #[runtime::derive(RuntimeEvent, RuntimeError, RuntimeOrigin, RuntimeCall)]

        pub  struct Test;

        #[runtime::pallet_index(0)]
        pub  type System = frame_system::Pallet<Test>;

        #[runtime::pallet_index(1)]
        pub  type Janus = pallet_janus::Pallet<Test>;
        
        #[runtime::pallet_index(2)]
        pub  type Balances = pallet_balances::Pallet<Test>;
    }
    
    pub fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default().build_storage()
            .unwrap();
        
        pallet_balances::GenesisConfig::<Test> {
            balances: vec![(1, 100_000) , (2, 100_000)],
        }.build_storage().unwrap().assimilate_storage(&mut t).unwrap();
            

        t.into()
    }
    
    #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type Nonce = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
        type Block = frame_system::mocking::MockBlock<Test>;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = ();
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
        type ExtensionsWeightInfo = ();
        type RuntimeTask = ();
        type RuntimeCall = RuntimeCall;
    }




    impl pallet_janus::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
    }
    
    impl pallet_balances::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
        type Balance = u128;
        type DustRemoval = ();
        type ExistentialDeposit = frame_support::traits::ConstU128<1>;
        type AccountStore = System;
        type ReserveIdentifier = [u8; 8];
        type RuntimeHoldReason = ();
        type RuntimeFreezeReason = ();
        type FreezeIdentifier = ();
        type MaxLocks = ();
        type MaxReserves = ();
        type MaxFreezes = ();
        type DoneSlashHandler = ();
    }
    
    
    
    
    
}