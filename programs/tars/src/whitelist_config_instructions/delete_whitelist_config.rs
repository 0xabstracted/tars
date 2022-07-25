use crate::whitelist_config::*;
use anchor_lang::prelude::*;
use common::close_account;

#[derive(Accounts)]
pub struct DeleteWhitelistConfig<'info> {
    #[account(mut, has_one = tars_creator)]
    whitelist_config: Account<'info, WhitelistConfig>,
    #[account(mut)]
    tars_creator: Signer<'info>,
}

pub fn handle_delete_whitelist_config(ctx: Context<DeleteWhitelistConfig>) -> Result<()> {
    close_account(
        &mut ctx.accounts.whitelist_config.to_account_info(),
        &mut ctx.accounts.tars_creator.to_account_info(),
    )?;

    Ok(())
}
