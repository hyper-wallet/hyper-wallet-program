use anchor_lang::prelude::*;
use instructions::*;
pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("HYPERhd7VFrTzbRLyGsRcGQZkSfaKUGKAY8XDbaY5AgL");

#[program]
pub mod hyper_wallet_program {
    use super::*;

    pub fn create_hyper_wallet(ctx: Context<CreateHyperWallet>) -> Result<()> {
        instructions::create_hyper_wallet::create_hyper_wallet(ctx)
    }

    pub fn transfer_lamports(ctx: Context<TransferLamports>, lamports: u64) -> Result<()> {
        instructions::transfer_lamports::transfer_lamports(ctx, lamports)
    }

    pub fn transfer_spl(ctx: Context<TransferSPL>, amount: u64) -> Result<()> {
        instructions::transfer_spl::transfer_spl(ctx, amount)
    }

    pub fn enable_whitelist(ctx: Context<EnableWhiteList>) -> Result<()> {
        instructions::whitelist::enable_whitelist(ctx)
    }

    pub fn disable_whitelist(ctx: Context<DisableWhiteList>) -> Result<()> {
        instructions::whitelist::disable_whitelist(ctx)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhiteList>, address: Pubkey) -> Result<()> {
        instructions::whitelist::add_to_whitelist(ctx, address)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhiteList>, address: Pubkey) -> Result<()> {
        instructions::whitelist::remove_from_whitelist(ctx, address)
    }

    pub fn enable_otp(ctx: Context<EnableOtp>) -> Result<()> {
        instructions::otp::enable_otp(ctx)
    }

    pub fn disable_otp(ctx: Context<DisableOtp>) -> Result<()> {
        instructions::otp::disable_otp(ctx)
    }

    pub fn set_spending_limit_lamports(
        ctx: Context<SetSpendingLimitLamports>,
        amount: u64,
    ) -> Result<()> {
        instructions::spending_limit::set_spending_limit_lamports(ctx, amount)
    }

    pub fn remove_spending_limit_lamports(ctx: Context<RemoveSpendingLimitLamports>) -> Result<()> {
        instructions::spending_limit::remove_spending_limit_lamports(ctx)
    }

    pub fn set_spending_limit_spl(
        ctx: Context<SetSpendingLimitSpl>,
        raw_amount: u64,
    ) -> Result<()> {
        instructions::spending_limit::set_spending_limit_spl(ctx, raw_amount)
    }

    pub fn remove_spending_limit_spl(ctx: Context<RemoveSpendingLimitSpl>) -> Result<()> {
        instructions::spending_limit::remove_spending_limit_spl(ctx)
    }
}
