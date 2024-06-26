use crate::errors::HyperWalletError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

pub fn transfer_spl(ctx: Context<TransferSPL>, raw_amount: u64) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.hyper_wallet;
    let from_ata = &mut ctx.accounts.from_ata;
    let to_ata = &mut ctx.accounts.to_ata;
    let approver = &mut ctx.accounts.approver;

    require!(
        hyper_wallet.is_valid_approver(approver.key()),
        HyperWalletError::InvalidApprover
    );

    let bump_vector = ctx.bumps.hyper_wallet.to_le_bytes();
    let binding = ctx.accounts.owner.key();
    let inner = vec![binding.as_ref(), bump_vector.as_ref()];
    let outer = vec![inner.as_slice()];

    let transfer_instruction = anchor_spl::token::Transfer {
        from: from_ata.to_account_info(),
        to: to_ata.to_account_info(),
        authority: hyper_wallet.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );
    anchor_spl::token::transfer(cpi_ctx, raw_amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct TransferSPL<'info> {
    #[account(mut, seeds = [owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    /// CHECK:
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub approver: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
