use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata,
    },
    token::{revoke, Mint, Revoke, Token, TokenAccount},
};

use crate::{
    error::StakeError,
    state::{StakeConfig, UserAccount, StakeAccount},
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut, //the UserAccount needs to be mut as we will be changing the states of it
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        close = user,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let time_elapsed =((Clock::get()?.unix_timestamp-self.stake_account.staked_at) / 86400) as u32;

        require!(time_elapsed > self.config.freeze_period, StakeError::FreezePeriodNotExpired);

        self.user_account.points += (self.config.points_per_stake as u32) * time_elapsed;

        let program = self.token_program.to_account_info();
        let accounts = ThawDelegatedAccountCpiAccounts {
            delegate: &self.stake_account.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.mint.to_account_info(),
            token_account: &self.user_mint_ata.to_account_info(),
            token_program: &self.token_program.to_account_info()
        };

        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];
        let signers_seeds = &[&seeds[..]];
        ThawDelegatedAccountCpi::new(&self.metadata_program.to_account_info(), accounts).invoke_signed(signers_seeds)?;

        let account = Revoke {
            authority: self.user.to_account_info(),
            source: self.user_mint_ata.to_account_info(),
        };

        let ctx = CpiContext::new(program, account);
        revoke(ctx)?;

        self.user_account.amount_staked -= 1;

        Ok(())
    }
}