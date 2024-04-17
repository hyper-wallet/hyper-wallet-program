use anchor_lang::prelude::*;
use crate::state::hyper_wallet::*;

pub fn transfer_lamports(ctx: Context<TransferLamports>, lamports: u64) -> Result<()> {
    // The code below normally work, but it wont work in this situation
    // Explanation below
    // ---
    // let smart_wallet = &mut ctx.accounts.from_smart_wallet;
    // let to= &mut ctx.accounts.to;
    //
    // let bump_vector = ctx.bumps.from_smart_wallet.to_le_bytes();
    // let binding = ctx.accounts.owner.key();
    // let inner = vec![binding.as_ref(), bump_vector.as_ref()];
    // let outer = vec![inner.as_slice()];
    //
    // let transfer_instruction = Transfer{
    //     from: smart_wallet.to_account_info(),
    //     to: to.to_account_info(),
    // };
    // let cpi_ctx = CpiContext::new_with_signer(
    //     ctx.accounts.system_program.to_account_info(),
    //     transfer_instruction,
    //     outer.as_slice()
    // );
    // anchor_lang::system_program::transfer(cpi_ctx, 100)?;
    // ---
    // Using system program instruction will result in
    // error: invalid program argument cause the PDA is
    // created and owned by our program, not the system program,
    // so the system program cant write to the PDA lamports
    // -> can implement transfer normally
    // Fortunately, Anchor has addressed this problem.
    // https://github.com/coral-xyz/anchor/pull/2552

    ctx.accounts.from_hyper_wallet.sub_lamports(lamports)?;
    ctx.accounts.to.add_lamports(lamports)?;
    Ok(())
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub from_hyper_wallet: Account<'info, HyperWallet>,
    #[account(mut)]
    /// CHECK:
    pub to: AccountInfo<'info>,
    pub hyper_wallet_owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
