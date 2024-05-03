use crate::state::hyper_wallet::*;
use anchor_lang::prelude::*;

pub fn close_hyper_wallet(ctx: Context<CloseHyperWallet>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CloseHyperWallet<'info> {
    #[account(mut, close = owner, seeds = [owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
