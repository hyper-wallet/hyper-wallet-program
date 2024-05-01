use crate::state::hyper_wallet::*;
use anchor_lang::prelude::*;

pub fn enable_otp(ctx: Context<EnableOtp>) -> Result<()> {
    ctx.accounts.hyper_wallet.otp_enabled = true;
    Ok(())
}

pub fn disable_otp(ctx: Context<DisableOtp>) -> Result<()> {
    ctx.accounts.hyper_wallet.otp_enabled = false;
    Ok(())
}

pub fn generate_otp(ctx: Context<GenerateOtp>) -> Result<()> {
    Ok(())
}

pub fn reset_otp(ctx: Context<ResetOtp>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct EnableOtp<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DisableOtp<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GenerateOtp<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetOtp<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}
