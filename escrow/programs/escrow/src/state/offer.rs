use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    // Details of the offer made, e.g. what who made it and what they want in return.
    // identifier of the offer
    pub id: u64,
    // who made the offer
    pub maker: Pubkey,
    // the token mint of the token being offered
    pub token_mint_a: Pubkey,
    // the token mint of the token being wanted
    pub token_mint_b: Pubkey,
    // the amount of token b being wanted
    pub token_b_wanted_amount: u64,
    // used to calculate the address for this account, we save it as a performance optimisation
    pub bump: u8,
}
