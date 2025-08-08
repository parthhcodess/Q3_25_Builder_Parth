use::anchor_lang::prelude::*;

#[account] // account for each user 
#[derive(InitSpace)]
pub struct UserAccount {
    pub points: u32,
    pub amount_staked: u8, // amount of nft staked
    pub bump: u8,
}