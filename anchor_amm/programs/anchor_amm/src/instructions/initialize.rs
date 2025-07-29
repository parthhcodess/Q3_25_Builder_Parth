use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        Mint,
        Token,
        TokenAccount
    }
};
use crate::state::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE, //8 bytes for custom discriminator
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = config.key(),
        seeds = [b"lp", config.key().as_ref()],
        bump
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>, 
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        seed: u64,
        fee: u16, //fee is in basis points(0 to 10000), so u16 is sufficient for that
        authority: Option<Pubkey>,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        // set inner is used so we don't miss out any of the parameters
        self.config.set_inner(
            Config {
                seed,
                authority,
                mint_x: self.mint_x.key(),
                mint_y: self.mint_y.key(),
                fee,
                locked: false,
                config_bump: bumps.config,
                lp_bump: bumps.mint_lp,
        });

        Ok(())
    }
}
