use crate::state::hyper_wallet::*;
use anchor_lang::prelude::*;

pub fn set_spending_limit_lamports(
    ctx: Context<SetSpendingLimitLamports>,
    amount: u64,
) -> Result<()> {
    Ok(())
}

pub fn remove_spending_limit_lamports(ctx: Context<RemoveSpendingLimitLamports>) -> Result<()> {
    Ok(())
}

pub fn set_spending_limit_spl(ctx: Context<SetSpendingLimitSpl>, raw_amount: u64) -> Result<()> {
    Ok(())
}

pub fn remove_spending_limit_spl(ctx: Context<RemoveSpendingLimitSpl>) -> Result<()> {
    Ok(())
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
}

#[derive(Accounts)]
pub struct RemoveSpendingLimitSpl<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}
