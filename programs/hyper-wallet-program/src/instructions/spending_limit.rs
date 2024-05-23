use crate::state::hyper_wallet::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub fn set_spending_limit_spl(ctx: Context<SetSpendingLimitSpl>, raw_amount: u64) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.hyper_wallet;
    let ata = &mut ctx.accounts.ata;
    let reset_period = 24 * 60 * 60;
    hyper_wallet.set_spending_limit(ata.key(), raw_amount, reset_period)
}

pub fn remove_spending_limit_spl(ctx: Context<RemoveSpendingLimitSpl>) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.hyper_wallet;
    let ata = &mut ctx.accounts.ata;
    hyper_wallet.remove_spending_limit(ata.key())
}

#[derive(Accounts)]
pub struct SetSpendingLimitLamports<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveSpendingLimitLamports<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetSpendingLimitSpl<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
    pub ata: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct RemoveSpendingLimitSpl<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
    pub ata: Account<'info, TokenAccount>,
}
