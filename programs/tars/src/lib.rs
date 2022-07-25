pub mod constants;
pub mod errors;
pub mod processor;
pub mod state;
pub mod utils;
pub mod wallet_whitelist;
pub mod whitelist_config;
pub mod whitelist_config_instructions;
pub mod whitelist_instructions;


use anchor_lang::prelude::*;
pub use errors::TarsError;
pub use processor::*;
pub use state::*;
pub use utils::*;
pub use whitelist_config::*;
pub use whitelist_config_instructions::*;
pub use whitelist_instructions::*;

declare_id!("CASE88YSoMZS8hFter4pzC9jC5g7RkPsjsaxH8uYzgGi");

#[program]
pub mod tars {

    use super::*;

    pub fn initialize_tars(
        ctx: Context<InitializeTars>,
        data: TarsData,
    ) -> Result<()> {
        handle_initialize_tars(ctx, data)
    }

    pub fn update_tars(ctx: Context<UpdateTars>, data: TarsData) -> Result<()> {
        handle_update_tars(ctx, data)
    }

    pub fn update_authority(
        ctx: Context<UpdateTars>,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        handle_update_authority(ctx, new_authority)
    }

    pub fn add_config_lines(
        ctx: Context<AddConfigLines>,
        index: u32,
        config_lines: Vec<ConfigLine>,
    ) -> Result<()> {
        handle_add_config_lines(ctx, index, config_lines)
    }

    pub fn set_collection(ctx: Context<SetCollection>) -> Result<()> {
        handle_set_collection(ctx)
    }

    pub fn remove_collection(ctx: Context<RemoveCollection>) -> Result<()> {
        handle_remove_collection(ctx)
    }

    pub fn mint_nft<'info>(
        ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>,
        creator_bump: u8,
    ) -> Result<()> {
        handle_mint_nft(ctx, creator_bump)
    }

    pub fn wl_mint_nft<'info>(
        ctx: Context<'_, '_, '_, 'info, WhitelistMintNFT<'info>>,
        creator_bump: u8,
    ) -> Result<()> {
        handle_whitelist_mint_nft(ctx, creator_bump)
    }

    pub fn set_collection_during_mint(ctx: Context<SetCollectionDuringMint>) -> Result<()> {
        handle_set_collection_during_mint(ctx)
    }

    pub fn withdraw_funds<'info>(ctx: Context<WithdrawFunds<'info>>) -> Result<()> {
        handle_withdraw_funds(ctx)
    }

    pub fn create_whitelist_account(
        ctx: Context<CreateWhitelistAccount>,
        whitelist_type: String,
    ) -> Result<()> {
        handler_create_whitelist_account(ctx, whitelist_type)
    }

    pub fn delete_whitelist_account(ctx: Context<DeleteWhitelistAccount>) -> Result<()> {
        handle_delete_whitelist_account(ctx)
    }

    pub fn create_whitelist_config(
        ctx: Context<CreateWhitelistConfig>,
        wl_start_time_4_wl_spots: u64,
        wl_start_time_4_wl_mp: u64,
        wl_start_time_4_wl_st: u64,
        wl_start_time_3_wl_spots: u64,
        wl_start_time_3_wl_mp: u64,
        wl_start_time_3_wl_st: u64,
        wl_start_time_2_wl_spots: u64,
        wl_start_time_2_wl_mp: u64,
        wl_start_time_2_wl_st: u64,
        wl_start_time_1_wl_spots: u64,
        wl_start_time_1_wl_mp: u64,
        wl_start_time_1_wl_st: u64,
    ) -> Result<()> {
        handler_create_whitelist_config(
            ctx,
            wl_start_time_4_wl_spots,
            wl_start_time_4_wl_mp,
            wl_start_time_4_wl_st,
            wl_start_time_3_wl_spots,
            wl_start_time_3_wl_mp,
            wl_start_time_3_wl_st,
            wl_start_time_2_wl_spots,
            wl_start_time_2_wl_mp,
            wl_start_time_2_wl_st,
            wl_start_time_1_wl_spots,
            wl_start_time_1_wl_mp,
            wl_start_time_1_wl_st,
        )
    }

    pub fn delete_whitelist_config(ctx: Context<DeleteWhitelistConfig>) -> Result<()> {
        handle_delete_whitelist_config(ctx)
    }

    pub fn update_whitelist_config(
        ctx: Context<UpdateWhitelistConfig>,
        wl_start_time_4_wl_spots: u64,
        wl_start_time_4_wl_mp: u64,
        wl_start_time_4_wl_st: u64,
        wl_start_time_3_wl_spots: u64,
        wl_start_time_3_wl_mp: u64,
        wl_start_time_3_wl_st: u64,
        wl_start_time_2_wl_spots: u64,
        wl_start_time_2_wl_mp: u64,
        wl_start_time_2_wl_st: u64,
        wl_start_time_1_wl_spots: u64,
        wl_start_time_1_wl_mp: u64,
        wl_start_time_1_wl_st: u64,
    ) -> Result<()> {
        handler_update_whitelist_config(
            ctx,
            wl_start_time_4_wl_spots,
            wl_start_time_4_wl_mp,
            wl_start_time_4_wl_st,
            wl_start_time_3_wl_spots,
            wl_start_time_3_wl_mp,
            wl_start_time_3_wl_st,
            wl_start_time_2_wl_spots,
            wl_start_time_2_wl_mp,
            wl_start_time_2_wl_st,
            wl_start_time_1_wl_spots,
            wl_start_time_1_wl_mp,
            wl_start_time_1_wl_st,
        )
    }
}
