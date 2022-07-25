use crate::wallet_whitelist::WLType;
use anchor_lang::prelude::*;

//#[proc_macros::assert_size(128)]
#[repr(C)]
#[account]
#[derive(Debug)]
pub struct WhitelistConfig {
    pub whitelist_schedule: WhitelistSchedule,
    pub tars_creator: Pubkey, //32
}

//#[proc_macros::assert_size(32)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize, PartialEq, PartialOrd)]
pub struct WhitelistTierConfig {
    pub whitelist_type: WLType,
    pub number_of_whitelist_spots_total: u64,
    pub discounted_mint_price: u64,
    pub whitelist_mint_start_time: u64,
}

//#[proc_macros::assert_size(96)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize, PartialEq, PartialOrd)]
pub struct WhitelistSchedule {
    pub wl_start_time_4: WhitelistTierConfig,
    pub wl_start_time_3: WhitelistTierConfig,
    pub wl_start_time_2: WhitelistTierConfig,
    pub wl_start_time_1: WhitelistTierConfig,
}
impl WhitelistSchedule {
    pub fn verify_schedule_invariants(&self) {
        assert!(
            self.wl_start_time_1.whitelist_mint_start_time
                >= self.wl_start_time_2.whitelist_mint_start_time
        );
        assert!(
            self.wl_start_time_2.whitelist_mint_start_time
                >= self.wl_start_time_3.whitelist_mint_start_time
        );
    }
}
