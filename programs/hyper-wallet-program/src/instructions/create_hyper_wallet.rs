use crate::state::hyper_wallet::*;
use anchor_lang::prelude::*;

pub fn create_hyper_wallet(ctx: Context<CreateHyperWallet>) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.hyper_wallet;
    hyper_wallet.owner = ctx.accounts.owner.key();
    hyper_wallet.whitelist_enabled = false;
    hyper_wallet.whitelisted_addresses = vec![];
    hyper_wallet.otp_enabled = false;
    hyper_wallet.spending_limit = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateHyperWallet<'info> {
    #[account(init, payer = rent_payer, space = 500, seeds = [owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
