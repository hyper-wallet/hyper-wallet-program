use crate::state::*;
use anchor_lang::prelude::*;

pub fn create_hyper_business_wallet(
    ctx: Context<CreateHyperBusinessWallet>,
    members: Vec<Pubkey>,
    threshold: u8,
    bump: u8,
) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    hyper_business_wallet.creator = ctx.accounts.creator.key();
    hyper_business_wallet.members = members;
    hyper_business_wallet.threshold = threshold;
    hyper_business_wallet.bump = bump;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateHyperBusinessWallet<'info> {
    #[account(init, payer = rent_payer, space = HyperBusinessWallet::size(5), seeds = [creator.key().as_ref()], bump)]
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
