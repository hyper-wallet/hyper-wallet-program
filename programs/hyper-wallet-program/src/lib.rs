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

    pub fn enable_white_list(ctx: Context<EnableWhiteList>) -> Result<()> {
        instructions::white_list::enable_white_list(ctx)
    }

    pub fn disable_white_list(ctx: Context<DisableWhiteList>) -> Result<()> {
        instructions::white_list::disable_white_list(ctx)
    }

    pub fn add_to_white_list(ctx: Context<AddToWhiteList>, address: Pubkey) -> Result<()> {
        instructions::white_list::add_to_white_list(ctx, address)
    }

    pub fn remove_from_white_list(
        ctx: Context<RemoveFromWhiteList>,
        address: Pubkey,
    ) -> Result<()> {
        instructions::white_list::remove_from_white_list(ctx, address)
    }
}
