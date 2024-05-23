use std::vec;

use crate::state::hyper_wallet::*;
use anchor_lang::prelude::*;
use rs_merkle::{algorithms::Sha256, MerkleProof};

pub fn enable_otp(ctx: Context<EnableOtp>) -> Result<()> {
    ctx.accounts.hyper_wallet.otp_enabled = true;
    Ok(())
}

pub fn disable_otp(ctx: Context<DisableOtp>) -> Result<()> {
    ctx.accounts.hyper_wallet.otp_enabled = false;
    Ok(())
}

pub fn set_up_otp(ctx: Context<SetUpOtp>, set_up_otp_params: SetUpOtpParams) -> Result<()> {
    ctx.accounts.hyper_wallet.otp_init_time = set_up_otp_params.init_time;
    ctx.accounts.hyper_wallet.otp_root = set_up_otp_params.root;
    ctx.accounts.hyper_wallet.otp_enabled = true;
    Ok(())
}

pub fn verify_otp(ctx: Context<VerifyOtp>, verify_otp_params: VerifyOtpParams) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let init_time = ctx.accounts.hyper_wallet.otp_init_time;
    let interval = ((current_time - init_time as i64) / 1) as usize;
    let proof_hash_copy = verify_otp_params.proof_hash.clone();
    let leave_hash = verify_otp_params.otp_hash;
    let root = ctx.accounts.hyper_wallet.otp_root;
    let proof = MerkleProof::<Sha256>::new(proof_hash_copy);
    let indices_to_prove = vec![interval];
    let valid = proof.verify(root, &indices_to_prove, &[leave_hash], usize::pow(2, 10));
    msg!("Valid: {}", valid.to_string());
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetUpOtpParams {
    pub init_time: u32,
    pub root: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VerifyOtpParams {
    pub otp_hash: [u8; 32],
    pub proof_hash: Vec<[u8; 32]>,
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
pub struct SetUpOtp<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyOtp<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}
