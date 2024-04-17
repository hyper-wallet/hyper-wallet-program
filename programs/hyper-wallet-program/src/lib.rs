use anchor_lang::prelude::*;

declare_id!("427T48CftuHif3sRLvaZp859Dzk1YCzUZAg2ooFCdiVK");

#[program]
pub mod hyper_wallet_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
