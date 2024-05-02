use anchor_lang::prelude::*;
use rs_merkle::{algorithms::Sha256, MerkleProof};

#[account]
pub struct HyperWallet {
    pub owner: Pubkey,
    pub whitelist_enabled: bool,
    pub whitelisted_addresses: Vec<Pubkey>,
    pub otp_enabled: bool,
    pub otp_root: [u8; 32],
    pub otp_init_time: u32,
    pub spending_limit: u64,
}

impl HyperWallet {
    pub fn validate_otp(&self, otp_hash: [u8; 32], proof_hash: Vec<[u8; 32]>) -> Result<bool> {
        if self.otp_enabled == false {
            return Ok(true);
        }

        let current_time = Clock::get()?.unix_timestamp;
        let init_time = self.otp_init_time;
        let interval = ((current_time - init_time as i64) / 1) as usize;
        let proof_hash_copy = proof_hash.clone();
        let leave_hash = otp_hash;
        let root = self.otp_root;
        let proof = MerkleProof::<Sha256>::new(proof_hash_copy);
        let indices_to_prove = vec![interval];
        Ok(proof.verify(root, &indices_to_prove, &[leave_hash], usize::pow(2, 10)))
    }
}
