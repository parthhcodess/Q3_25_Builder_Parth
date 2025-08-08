use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::state::{StakeConfig, UserAccount};

// CLAIM INSTRUCTION: Convert accumulated points into reward tokens
// This is the final step in the staking lifecycle: stake -> earn -> claim
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // USER ACCOUNT: Contains the points earned from staking (will be modified to reset points)
    // Seeds Pattern: ["user", user_wallet_address] - Unique per user
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    // REWARDS MINT: The "token factory" that creates reward tokens
    // Seeds pattern: ["rewards", config_address] - controlled by the config account
    // This account will be modified to increase the total supply when minting
    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub rewards_mint: Account<'info, Mint>,

    // CONFIG: Global settings - serves as the mint authority for the rewards
    // Seeds pattern: ["config"] - single global configuration
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    // REWARDS ATA: User's personal "reward token wallet"
    // Associated Token Account - automatically derived address for user + mint combo
    // init_if_needed: creates the account if it doesn't exists yet
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        // STEP 1: SET UP CPI TO TOKEN PROGRAM
        // We are going to call the Token Program's "mint_to" function
        let cpi_program = self.token_program.to_account_info();
        
        // STEP 2: CREATE SIGNER SEEDS FOR CONFIG AUTHORITY
        // The config account is the mint authority, but it is a PDA
        // PDAs can't sign directly, we need to use the seeds to sign on behalf of the pda
        // this is same pattern used in the stake.rs for freeze operations
        let seeds = &[
            b"config".as_ref(),
            &[self.config.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        // STEP 3: DEFINE THE MINT OPERATION
        // This tells the token program to mint new tokens
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),  //WHAT: which token type to mint
            to: self.rewards_ata.to_account_info(),     //WHERE: User's token acc
            authority: self.config.to_account_info()    //WHO: config has the permission to mint
        };

        // new_with_signer - we are using pda signing not user signing like in stake.rs
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts, 
            signer_seeds
        );

        // STEP 4: EXECUTE THE MINT WITH DECIMAL OPERATION
        // Key insight: self.user_account.points = human-readable number (5 points)
        // But tokens need "atomic units" based on decimals (5,000,000 for 6 decimals)
        //
        // Formula: points * 10^decimals = atomic_units
        // Example: 5 points * 10^6 = 5,000,000 atomic units = 5.000000 tokens
        mint_to(
            cpi_ctx, 
            self.user_account.points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32)
        )?;

        // STEP 5: RESET USER POINTS TO 0
        // Points have been cashed in for token, so we can clear the balance now
        // This will prevent the double claiming of the same points
        self.user_account.points = 0;

        Ok(())
    }
}