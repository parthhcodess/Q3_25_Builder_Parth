use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed: u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool, //it is set in place to stop any kind of activity on AMM(eg; vulnerability)
    pub config_bump: u8,
    pub lp_bump: u8,
}