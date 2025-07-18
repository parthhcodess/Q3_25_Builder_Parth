use anchor_lang::{prelude::*};
use crate::{
    error::ErrorCode, 
    state::Offer, 
    handlers::{transfer_tokens, close_token_account},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface}
};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    // TakeOffer (in capitals) is a struct of names accounts that the
    // take_offer() function will use.
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,

    // as bob is signing so he need to pay the trans. fee ie mut.
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,

    pub token_mint_b: InterfaceAccount<'info, Mint>,

    // this is where bob is recieving the tokens where alice is sending
    #[account(
        init_if_needed, //account may or not already exists
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    // this is where bob is sending his tokens to alice
    #[account(
        mut, // bcoz balance is going to chnge
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_b: InterfaceAccount<'info, TokenAccount>,

    // maybe alice didnt have any bonk so created it
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_b,
        seeds = [b"offer",  offer.id.to_le_bytes().as_ref()],
        bump = offer.bump,
    )]
    offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>
}

// Handle the take offer instruction by:
// 1. Sending the wanted tokens from the taker to the maker
// 2. Withdrawing the offered tokens from the vault to the taker and closing the vault
pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {    
    // since the offer account owns the vault, we will say
    // there is one signer (the offer), with the seeds of the specific offer account
    // we can use these signer seeds to withdraw the token from the vault
    let offer_account_seeds = &[
        b"offer",
        &context.accounts.offer.id.to_le_bytes()[..],
        &[context.accounts.offer.bump],
    ];
    let signer_seeds = Some(&offer_account_seeds[..]);

    // withdraw the offered tokens from the vault to the taker
    transfer_tokens(
        &context.accounts.vault,
        &context.accounts.taker_token_account_a,
        &context.accounts.vault.amount,
        &context.accounts.token_mint_a,
        &context.accounts.offer.to_account_info(),
        &context.accounts.token_program,
        signer_seeds,
    )
    .map_err(|_| ErrorCode::FailedVaultWithdrawal)?;

    close_token_account(
        &context.accounts.vault,
        &context.accounts.taker.to_account_info(),
        &context.accounts.offer.to_account_info(),
        &context.accounts.token_program,
        signer_seeds,
    )
    .map_err(|_| ErrorCode::FailedVaultClosure)?;
    Ok(())
}
