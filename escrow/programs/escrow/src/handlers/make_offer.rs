use anchor_lang::prelude::*;
use crate::{error::ErrorCode, state::Offer};
use super::shared::transfer_tokens;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
#[instruction(id: u64)] //struct as access to this id   
pub struct MakeOffer<'info> {
    // MakeOffer (in capitals) is a struct of names accounts that the
    // make_offer() function will use.
    // used to manage associated token accounts
    // ie where a wallet holes a specific type of token
    pub associated_token_program: Program<'info, AssociatedToken>,
    // work with either the classic token program or
    // the newer token extensions program
    pub token_program: Interface<'info, TokenInterface>,
    // used to create accounts
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub maker: Signer<'info>,

    // this is alice's usdc
    #[account(
        mint::token_program = token_program
    )]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    // alice's token account
    #[account(
        mut,
        associated_token::mint = token_mint_a, //mint matches token mint a
        associated_token::authority = maker, //check that acc is owned by alice
        associated_token::token_program = token_program, //token program is same elsewhere
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init, //will fail if offer already exists or we run out of tests
        payer = maker, //alice is making this account
        space = Offer::DISCRIMINATOR.len() + Offer::INIT_SPACE,
        seeds = [b"offer", id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    // file account need for make offer
    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
}

// Handle the make offer instruction by:
// 1. Moving the tokens from the maker's ATA to the vault
// 2. Saving the details of the offer to the offer account
pub fn make_offer(
    context: Context<MakeOffer>, //every anchor inst handler starts with context
    id: u64, //offer id
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // validate amounts
    require!(token_a_offered_amount > 0, ErrorCode::InvalidAmount);
    require!(token_b_wanted_amount > 0, ErrorCode::InvalidAmount);

    // validate token mints are different
    // i dont want 9 usdc in exchange of 10 usdc
    require!(
        context.accounts.token_mint_a.key() != context.accounts.token_mint_b.key(),
        ErrorCode::InvalidTokenMint
    );

    // move the tokens from the maker's ATA to the vault
    transfer_tokens(
        &context.accounts.maker_token_account_a,
        &context.accounts.vault,
        &token_a_offered_amount,
        &context.accounts.token_mint_a,
        &context.accounts.maker.to_account_info(),
        &context.accounts.token_program,
        None,
    )
    .map_err(|_| ErrorCode::InsufficientMakerBalance)?; //nice error message

    // save the details of the offer to the offer account
    context.accounts.offer.set_inner(Offer {
        id,
        maker: context.accounts.maker.key(),
        token_mint_a: context.accounts.token_mint_a.key(),
        token_mint_b: context.accounts.token_mint_b.key(),
        token_b_wanted_amount,
        bump: context.bumps.offer,
    });

    Ok(())
}
