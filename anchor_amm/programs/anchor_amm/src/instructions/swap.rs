use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::error::AmmError;
use crate::state::Config;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    // amm pool configuration acc that we initialized in instruction
    #[account(
        has_one = mint_x, //mints are same as in the config we are dealing with(to compare them)
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed, //possibility that user might not have that token
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_token_account_x: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_token_account_y: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount > 0, AmmError::InvalidAmount);

        // we are initializing the curve by determining the current state
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            self.config.fee,
            None, //we need it to calculate b/w tokens and precision does the calculations as we are dealing with different tokens. "None is given as it is defaults to 6 only"
        )
        .map_err(AmmError::from)?;

        // for which token is being swapped in 
        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        // calculating the swap amts using constant product formula
        let res = curve.swap(p, amount, min).map_err(AmmError::from)?;

        // validating the cal amounts are valid or not
        require!(res.deposit != 0, AmmError::InvalidAmount);
        require!(res.withdraw != 0, AmmError::InvalidAmount);

        // deposit the tokens
        self.deposit_tokens(is_x, res.deposit);
        // withdraw the tokens
        self.withdraw_tokens(is_x, res.withdraw);

        Ok(())
    }

    pub fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {

        let(from, to) = match is_x {
            true => (
                self.user_token_account_x.to_account_info(),
                self.vault_x.to_account_info(),
            ),
            false => (
                self.user_token_account_y.to_account_info(),
                self.vault_y.to_account_info(),
            )
        };

        let cpi_program = self.token_program.to_account_info();

        let accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, accounts); //we are using new coz user is the one holding the tokens and transfering

        transfer(cpi_ctx, amount); 
        // the difference in TransferChecked is it takes extra constraints also as mint

        Ok(())
    }

    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {

        let(from, to) = match is_x {
            // when the x is true we need to withdraw y, that's why the inversion of accounts are here
            true => (
                self.vault_y.to_account_info(),
                self.user_token_account_y.to_account_info(),
            ),
            false => (
                self.vault_x.to_account_info(),
                self.user_token_account_x.to_account_info(),
            )
        };

        let cpi_program = self.token_program.to_account_info();

        let accounts = Transfer {
            from,
            to,
            authority: self.config.to_account_info(),
        };

        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}