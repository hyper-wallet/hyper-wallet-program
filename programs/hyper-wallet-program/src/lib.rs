use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;
pub mod state;
pub mod errors;

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
}
