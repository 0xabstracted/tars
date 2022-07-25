use anchor_lang::prelude::*;
use std::fmt::Debug;

//#[proc_macros::assert_size(4)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize, PartialEq, PartialOrd)]
pub enum WLType {
    Null,
    One,
    Two,
    Three,
    Four,
}

//#[proc_macros::assert_size(88)]
#[repr(C)]
#[account]
#[derive(Debug)]
pub struct WalletWhitelist {
    pub tars_creator: Pubkey,               //32
    pub whitelisted_address: Pubkey,             //32
    pub whitelist_type: WLType,                  //4
    pub number_of_whitelist_spots_per_user: u64, //8
    pub discounted_mint_price: u64,              //8
    pub whitelist_mint_start_time: u64,          //8
}
