use anchor_lang::{prelude::*, Discriminator};
use mpl_token_metadata::state::{MAX_CREATOR_LIMIT, MAX_SYMBOL_LENGTH};
use spl_token::state::Mint;

use crate::{
    assert_initialized, assert_owned_by, cmp_pubkeys,
    constants::{CONFIG_ARRAY_START, CONFIG_LINE_SIZE},
    TarsError, Tars, TarsData,
};

/// Create a new tars.
#[derive(Accounts)]
#[instruction(data: TarsData)]
pub struct InitializeTars<'info> {
    /// CHECK: account constraints checked in account trait
    #[account(zero, rent_exempt = skip, constraint = tars.to_account_info().owner == program_id && tars.to_account_info().data_len() >= get_space_for_tars(data)?)]
    tars: UncheckedAccount<'info>,
    /// CHECK: wallet can be any account and is not written to or read
    wallet: UncheckedAccount<'info>,
    /// CHECK: authority can be any account and is not written to or read
    authority: UncheckedAccount<'info>,
    payer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_tars(
    ctx: Context<InitializeTars>,
    data: TarsData,
) -> Result<()> {
    let tars_account = &mut ctx.accounts.tars;

    if data.uuid.len() != 6 {
        return err!(TarsError::UuidMustBeExactly6Length);
    }

    let mut tars = Tars {
        data,
        authority: ctx.accounts.authority.key(),
        wallet: ctx.accounts.wallet.key(),
        token_mint: None,
        items_redeemed: 0,
    };

    if !ctx.remaining_accounts.is_empty() {
        let token_mint_info = &ctx.remaining_accounts[0];
        let _token_mint: Mint = assert_initialized(token_mint_info)?;
        let token_account: spl_token::state::Account = assert_initialized(&ctx.accounts.wallet)?;

        assert_owned_by(token_mint_info, &spl_token::id())?;
        assert_owned_by(&ctx.accounts.wallet, &spl_token::id())?;

        if !cmp_pubkeys(&token_account.mint, &token_mint_info.key()) {
            return err!(TarsError::MintMismatch);
        }

        tars.token_mint = Some(*token_mint_info.key);
    }

    let mut array_of_zeroes = vec![];
    while array_of_zeroes.len() < MAX_SYMBOL_LENGTH - tars.data.symbol.len() {
        array_of_zeroes.push(0u8);
    }
    let new_symbol =
        tars.data.symbol.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();
    tars.data.symbol = new_symbol;

    // - 1 because we are going to be a creator
    if tars.data.creators.len() > MAX_CREATOR_LIMIT - 1 {
        return err!(TarsError::TooManyCreators);
    }

    let mut new_data = Tars::discriminator().try_to_vec().unwrap();
    new_data.append(&mut tars.try_to_vec().unwrap());
    let mut data = tars_account.data.borrow_mut();
    // god forgive me couldnt think of better way to deal with this
    for i in 0..new_data.len() {
        data[i] = new_data[i];
    }

    // only if we are not using hidden settings we will have space for
    // the config lines
    if tars.data.hidden_settings.is_none() {
        let vec_start = CONFIG_ARRAY_START
            + 4
            + (tars.data.items_available as usize) * CONFIG_LINE_SIZE;
        let as_bytes = (tars
            .data
            .items_available
            .checked_div(8)
            .ok_or(TarsError::NumericalOverflowError)? as u32)
            .to_le_bytes();
        for i in 0..4 {
            data[vec_start + i] = as_bytes[i]
        }
    }

    Ok(())
}

fn get_space_for_tars(data: TarsData) -> Result<usize> {
    let num = if data.hidden_settings.is_some() {
        CONFIG_ARRAY_START
    } else {
        CONFIG_ARRAY_START
            + 4
            + (data.items_available as usize) * CONFIG_LINE_SIZE
            + 8
            + 2 * ((data
                .items_available
                .checked_div(8)
                .ok_or(TarsError::NumericalOverflowError)?
                + 1) as usize)
    };

    Ok(num)
}
