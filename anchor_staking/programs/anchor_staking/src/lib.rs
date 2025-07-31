use anchor_lang::prelude::*;

declare_id!("8gGNB1VCBEAUdzEoczhAToPHZg2JwKwcZjfQrB9R51VZ");

#[program]
pub mod anchor_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
