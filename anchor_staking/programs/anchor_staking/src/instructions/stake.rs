use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::{
    error::StakeError,
    state::{StakeConfig, UserAccount, StakeAccount},
};

/*
STAKE INSTRUCTION 
=================

ACCOUNTS NEEDED:
1. user: Signer - The person staking their NFT(must sign transaction)
2. mint: Account<Mint> - The specific NFT mint being staked
3. collection_mint: Account<Mint> - The collection this NFT must belong to
4. user_mint_ata: Account<TokenAccount> - User's ATA  holding the NFT
5. metadata: Account<MetadataAccount> - NFT's metaplex metadata (PDA: ["metadata", metadata_program, mint]) to ensure its part of the collection and verified or not
6. edition: Account<MetadataAccount> - NFT's master edition (PDA: ["metadata", metadata_program, mint]) to ensure its one of a kind
7. config: Account<StakeConfig> - Global staking rules (PDA: ["config"])
8. user_account: Account<UserAccount> - User's staking aggregation (PDA: ["user", user.key()])
9. stake_account: Account<StakeAccount> - New Account for this stake (PDA: ["stake", mint.key(), config.key()])
10. system_program: Program<System> - For account creation
11. token_program: Program<Token> - For token operations
12. metadata_program: Program<Metadata> - For NFT freezing

VALIDATIONS PERFORMED;
✅ white_check_mark: User signature verified
✅ NFT ownership (user_mint_ata.authority == user)
✅ Correct NFT in the wallet (user_mint_ata.mint == mint)
✅ Collection membership (metadata.collection.key == collection.mint)
✅ Collection verification (metadata.collection.verified == true)
✅ PDA derivations for all pdas
✅ User Account exists and is valid
✅ Config Account exists and it valid
✅ Staking limit check (user_account.amount_staked < config.max_stake)

EXECUTION FLOW:
1. Business Logic: Check user hasn't exceeded max stake limit
2. Create Record: Initialize StakeAccount with owner, mint, timestamp, and PDA bump
3. Grant Permission: Call Token Program's approve() to delegate NFT control to stake_account
4. Prepare PDA Signing: Generate signer seeds for stake_account PDA
5. Freeze NFT: Call metadata program's freeze_delegated_account() using invoke_signed()
6. Update Stats: Increment user_account.amount_staked counter
7. Success: Return Ok(()) indicating successful staking

SECURITY MODEL:
- Two-phase protection: Permission delegation + NFT freezing
- PDA-controlled accounts prevent unauthorized access
- Metaplex collection verification ensures legitimate NFTs only
- All state changes are atomic (succeed together or fail together)

AUTHORITY & OWNERSHIP DIAGRAM
=============================

                     👤 USER (SIGNER)
                    │
                    ├─ owns → 👛 user_mint_ata (TokenAccount)
                    │           │
                    │           ├─ holds → 🎨 NFT (amount: 1)
                    │           └─ authority: User ──┐
                    │                                │
                    └─ signs for → 💰 Transaction    │
                                                     │
    ╔══════════════════ BEFORE STAKING ═════════════▼═══════════╗
    ║  👛 user_mint_ata:                                        ║
    ║  • owner: User                                            ║
    ║  • delegate: None                 ← User has full control ║
    ║  • state: Normal                                          ║
    ╚═══════════════════════════════════════════════════════════╝
                                  │
                                  │ approve() call
                                  ▼
    ╔══════════════════ AFTER APPROVE ══════════════════════════╗
    ║  👛 user_mint_ata:                                        ║
    ║  • owner: User                                            ║
    ║  • delegate: stake_account     ← Staking program can act  ║
    ║  • state: Normal                                          ║
    ╚═══════════════════════════════════════════════════════════╝
                                  │
                                  │ freeze_delegated_account()
                                  ▼
    ╔══════════════════ AFTER FREEZE ═══════════════════════════╗
    ║  👛 user_mint_ata:                                        ║
    ║  • owner: User                                            ║
    ║  • delegate: stake_account     ← Program controls NFT     ║
    ║  • state: FROZEN               ← Nobody can transfer      ║
    ╚═══════════════════════════════════════════════════════════╝

PROGRAM OWNERSHIP HIERARCHY:
============================

🏢 TOKEN PROGRAM owns:
  ├─ mint (NFT Mint Account)
  └─ user_mint_ata (Token Account)

🏢 METADATA PROGRAM owns:
  ├─ metadata (PDA: ["metadata", metadata_program, mint])
  └─ edition (PDA: ["metadata", metadata_program, mint, "edition"])

🏢 OUR STAKING PROGRAM owns:
  ├─ config (PDA: ["config"])
  ├─ user_account (PDA: ["user", user.key()])
  └─ stake_account (PDA: ["stake", mint.key(), config.key()])


AUTHORITY FLOW DURING STAKING
=============================

1. User -> calls stake() -> Staking program
2. Staking Program -> approve(delegate) -> Token Program
3. Staking Program -> freeze_delegated_account() -> Metadata Program
4. Metadata Program -> set_frozen(true) -> user_mint_ata

WHO CAN DO WHAT:
================

BEFORE STAKING:
- User: ✅ Can Transfer NFT, ✅ Can approve delegates
- Programs: ❌ Cannot touch user's NFT

AFTER STAKING:
- User: ❌ Cannot transfer NFT, ❌ Cannot approve new delegates
- Staking Programs: ✅ Can Unfreeze NFT (via PDA signing)
- Other Programs: ❌ Cannot touch frozen NFT

FINAL STATE:
- NFT is frozen and cannot be transferred by anyone
- StakeAccount exists as proof of staking with timestamp
- User's stake count is incremented
- NFT begins earning rewards based on staked_at timestamp
- Only our staking program can reverse this state (UNSTAKING)
*/

#[derive(Accounts)]
pub struct Stake<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    // this is the pda that is created by metadata program not by us
    #[account(
        // seeds are deterministic, so the acc derived will determine the seeds and the program passed
        // for the metadata program, the seeds are of specific format only 
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(), //bcoz pda belongs to metadata program, it will assume the program is actually our own create program if not given
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(), //to check its part of the collection
        constraint = metadata.collection.as_ref().unwrap().verified == true, // to check its verified or not
    )]
    pub metadata: Account<'info, MetadataAccount>,
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
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        // checks the limit of NFTs staked by this user is not reached
        require!(self.user_account.amount_staked < self.config.max_stake, StakeError::MaxStakeReached);

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(), 
            mint: self.mint.key(), 
            staked_at: Clock::get()?.unix_timestamp, 
            bump: bumps.stake_account, 
        });

        let cpi_program= self.token_program.to_account_info();
        // Set up the approve call: "User gives staking program permission to control the NFT"
        let cpi_accounts = Approve {
            to: self.user_mint_ata.to_account_info(),          // The token account holding the NFT   
            delegate: self.stake_account.to_account_info(),    // WHO gets the authority (stake_account)
            authority: self.user.to_account_info()             // WHO is granting authority (user) 
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // THIS LINE TRANSFERS AUTHORITY: user_mint_ata.delegate = stake_account
        // After this call the staking program controls the NFT (but user still owns it)
        approve(cpi_ctx, 1)?; //Approve delegation of 1 token (the NFT)

        let mint_key = self.mint.key();
        let config_key = self.config.key();
        let seeds = &[
            b"stake",
            mint_key.as_ref(),
            config_key.as_ref(),
            &[self.stake_account.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.user_mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        FreezeDelegatedAccountCpi::new(
            metadata_program, 
            FreezeDelegatedAccountCpiAccounts { 
                delegate, 
                token_account, 
                edition, 
                mint, 
                token_program 
            }
        )
        .invoke_signed(signer_seeds)?;

        self.user_account.amount_staked += 1;

        Ok(())
    }
}