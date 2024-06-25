use crate::state::*;
use anchor_lang::prelude::*;

pub fn create_hyper_wallet(
    ctx: Context<CreateHyperWallet>,
    voters: Vec<Pubkey>,
    threshold: u8,
    bump: u8,
) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.hyper_wallet;
    hyper_wallet.owner = ctx.accounts.owner.key();
    hyper_wallet.voters = voters;
    hyper_wallet.threshold = threshold;
    hyper_wallet.bump = bump;

    Ok(())
}

pub fn close_hyper_wallet(_ctx: Context<CloseHyperWallet>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateHyperWallet<'info> {
    #[account(init, payer = rent_payer, space = HyperWallet::size(5), seeds = [owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseHyperWallet<'info> {
    #[account(mut, close = owner, seeds = [owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
