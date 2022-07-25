use crate::wallet_whitelist::WLType;
use crate::whitelist_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateWhitelistConfig<'info> {
    #[account(mut, has_one = tars_creator)]
    whitelist_config: Account<'info, WhitelistConfig>,
    tars_creator: Signer<'info>,
}

pub fn handler_update_whitelist_config(
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
    let whitelist_schedule = WhitelistSchedule {
        wl_start_time_4: WhitelistTierConfig {
            whitelist_type: WLType::Four,
            number_of_whitelist_spots_total: wl_start_time_4_wl_spots,
            discounted_mint_price: wl_start_time_4_wl_mp,
            whitelist_mint_start_time: wl_start_time_4_wl_st,
        },
        wl_start_time_3: WhitelistTierConfig {
            whitelist_type: WLType::Three,
            number_of_whitelist_spots_total: wl_start_time_3_wl_spots,
            discounted_mint_price: wl_start_time_3_wl_mp,
            whitelist_mint_start_time: wl_start_time_3_wl_st,
        },
        wl_start_time_2: WhitelistTierConfig {
            whitelist_type: WLType::Two,
            number_of_whitelist_spots_total: wl_start_time_2_wl_spots,
            discounted_mint_price: wl_start_time_2_wl_mp,
            whitelist_mint_start_time: wl_start_time_2_wl_st,
        },
        wl_start_time_1: WhitelistTierConfig {
            whitelist_type: WLType::One,
            number_of_whitelist_spots_total: wl_start_time_1_wl_spots,
            discounted_mint_price: wl_start_time_1_wl_mp,
            whitelist_mint_start_time: wl_start_time_1_wl_st,
        },
    };

    let whitelist_config = &mut ctx.accounts.whitelist_config;
    whitelist_schedule.verify_schedule_invariants();
    whitelist_config.whitelist_schedule = whitelist_schedule;
    Ok(())
}
