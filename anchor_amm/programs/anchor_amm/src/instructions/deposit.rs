use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        mint_to,
        transfer,
        Mint,
        MintTo,
        Token,
        TokenAccount,
        Transfer,
    }
};
use constant_product_curve::ConstantProduct;

use crate::{state::Config, error::AmmError};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config"],
        bump = config.config_bump,  
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut, //changing the supply
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        mut, //only mutable coz we transferring to the vault
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
        mut, 
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed, //we dk if it def exists or not(user may be depositinf for the 2nd time)
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {

    pub fn deposit(&mut self, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        // to make sure the amount is not 0 && the config is locked as false
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);

        // how many tokens of x and y to deposit for the amt the user claim as lp token
        let(x, y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 { //all are 0 coz to check whether somebody using lp for the first time
            true => (max_x, max_y), //we dont need to adhere to the curve mow
            false => {
                let amount = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount, 
                    self.vault_y.amount, 
                    self.mint_lp.supply, 
                    amount,
                    6, 
                )
                .unwrap();
                (amount.x, amount.y)
            }
        };

        // to check for the slippage 
        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);

        // deposit the token from the user to vault
        self.deposit_tokens(true, x);
        self.deposit_tokens(false, y);

        // mint lp tokens to user
        self.mint_lp_tokens(amount);

        Ok(())
    }
    pub fn deposit_tokens(
        &self,
        is_x: bool, //to avoid redundancy to not write for x and y separately
        amount: u64, 
    ) -> Result<()> {

        let(from ,to) = match is_x {
            true => (self.user_token_account_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_token_account_y.to_account_info(), self.vault_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };
        
        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(ctx, amount);
        Ok(())
    }

    pub fn mint_lp_tokens(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts =  MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_token_account_lp.to_account_info(),
            authority: self.config.to_account_info(), 
        };

        //the authority in this case is pda so we need signer seeds(new_with_signer)
        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(ctx, amount);

        Ok(())
    }
}