use::anchor_lang::prelude::*;

#[account] // to know who is the owner & what token are we dealing with
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub staked_at: i64,
    pub bump: u8,
}