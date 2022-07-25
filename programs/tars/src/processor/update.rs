use anchor_lang::prelude::*;

use crate::{
    constants::COLLECTIONS_FEATURE_INDEX, is_feature_active, TarsError, Tars,
    TarsData,
};

/// Update the tars state.
#[derive(Accounts)]
pub struct UpdateTars<'info> {
    #[account(
    mut,
    has_one = authority
    )]
    tars: Account<'info, Tars>,
    authority: Signer<'info>,
    /// CHECK: wallet can be any account and is not written to or read
    wallet: UncheckedAccount<'info>,
}

pub fn handle_update_authority(
    ctx: Context<UpdateTars>,
    new_authority: Option<Pubkey>,
) -> Result<()> {
    let tars = &mut ctx.accounts.tars;

    if let Some(new_auth) = new_authority {
        tars.authority = new_auth;
    }

    Ok(())
}

// updates without modifying UUID
pub fn handle_update_tars(
    ctx: Context<UpdateTars>,
    data: TarsData,
) -> Result<()> {
    let tars = &mut ctx.accounts.tars;

    if data.items_available != tars.data.items_available && data.hidden_settings.is_none()
    {
        return err!(TarsError::CannotChangeNumberOfLines);
    }

    if tars.data.items_available > 0
        && tars.data.hidden_settings.is_none()
        && data.hidden_settings.is_some()
    {
        return err!(TarsError::CannotSwitchToHiddenSettings);
    }

    let old_uuid = tars.data.uuid.clone();
    tars.wallet = ctx.accounts.wallet.key();
    if is_feature_active(&old_uuid, COLLECTIONS_FEATURE_INDEX) && !data.retain_authority {
        return err!(TarsError::TarsCollectionRequiresRetainAuthority);
    }
    tars.data = data;
    tars.data.uuid = old_uuid;

    if !ctx.remaining_accounts.is_empty() {
        tars.token_mint = Some(ctx.remaining_accounts[0].key())
    } else {
        tars.token_mint = None;
    }
    Ok(())
}
