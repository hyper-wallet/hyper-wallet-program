use anchor_lang::prelude::*;
use crate::state::hyper_wallet::*;
use anchor_spl::token::{Token, TokenAccount};

pub fn transfer_spl(ctx: Context<TransferSPL>, amount: u64) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.from_hyper_wallet;
    let hyper_wallet_ata = &mut ctx.accounts.from_hyper_wallet_ata;
    let to_ata= &mut ctx.accounts.to_ata;

    let bump_vector = ctx.bumps.from_hyper_wallet.to_le_bytes();
    let binding = ctx.accounts.hyper_wallet_owner.key();
    let inner = vec![binding.as_ref(), bump_vector.as_ref()];
    let outer = vec![inner.as_slice()];

    let transfer_instruction = anchor_spl::token::Transfer{
        from: hyper_wallet_ata.to_account_info(),
        to: to_ata.to_account_info(),
        authority: hyper_wallet.to_account_info()
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice()
    );
    anchor_spl::token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct TransferSPL<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub from_hyper_wallet: Account<'info, HyperWallet>,
    #[account(mut)]
    pub from_hyper_wallet_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK:
    pub to_ata: Account<'info, TokenAccount>,
    pub hyper_wallet_owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
