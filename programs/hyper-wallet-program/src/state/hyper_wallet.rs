use crate::errors::HyperWalletError;
use anchor_lang::prelude::*;
use rs_merkle::{algorithms::Sha256, MerkleProof};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SpendingLimit {
    pub ata: Pubkey,
    pub raw_amount: u64,
    pub raw_allowance_left: u64,
    pub last_reset: i64,
}

#[account]
pub struct HyperWallet {
    pub owner: Pubkey,
    pub whitelist_enabled: bool,
    pub whitelisted_addresses: Vec<Pubkey>,
    pub otp_enabled: bool,
    pub otp_root: [u8; 32],
    pub otp_init_time: u32,
    pub spending_limit_enabled: bool,
    pub spending_limits: Vec<SpendingLimit>,
}

impl HyperWallet {
    pub fn verify_receiver(&self, receiver: Pubkey) -> Result<()> {
        if self.whitelist_enabled == false {
            return Ok(());
        }

        let receiver_is_whitelisted =
            match self.whitelisted_addresses.iter().find(|&v| *v == receiver) {
                None => false,
                Some(_v) => true,
            };

        require!(
            receiver_is_whitelisted,
            HyperWalletError::AddressNotWhiteListed
        );

        Ok(())
    }
    // pub fn verify_spending_limit(&self, ata: Pubkey, raw_amount: u64) -> Result<()> {
    //     if self.spending_limit_enabled == false {
    //         return Ok(());
    //     }

    //     let mut spending_limit = match self.spending_limits.iter().find(|&v| v.ata == ata).as_mut()
    //     {
    //         None => return Ok(()),
    //         Some(v) => &v,
    //     };

    //     let current_time = Clock::get()?.unix_timestamp;
    //     if current_time - spending_limit.last_reset > 24 * 60 * 60 {
    //         &spending_limit.last_reset = current_time;
    //         spending_limit.raw_allowance_left = spending_limit.raw_amount;
    //     }

    //     require_gte!(
    //         raw_amount,
    //         spending_limit.raw_allowance_left,
    //         HyperWalletError::SpendingLimitExceeded
    //     );

    //     spending_limit.raw_allowance_left -= raw_amount;

    //     Ok(())
    // }

    pub fn verify_otp(
        &self,
        otp_hash: Option<[u8; 32]>,
        proof_hash: Option<Vec<[u8; 32]>>,
    ) -> Result<()> {
        if self.otp_enabled == false {
            return Ok(());
        }

        require!(otp_hash.is_some(), HyperWalletError::OtpIsRequired);
        require!(
            proof_hash.is_some(),
            HyperWalletError::OtpProofPathIsRequired,
        );

        let otp_hash = otp_hash.unwrap();
        let proof_hash = proof_hash.unwrap();

        let current_time = Clock::get()?.unix_timestamp;
        let init_time = self.otp_init_time;
        let interval = ((current_time - init_time as i64) / 30) as usize;

        let proof_hash_copy = proof_hash.clone();
        let proof = MerkleProof::<Sha256>::new(proof_hash_copy);
        let root = self.otp_root;
        let indices_to_prove = vec![interval];
        let leave_hash = otp_hash;

        require!(
            proof.verify(root, &indices_to_prove, &[leave_hash], usize::pow(2, 10)),
            HyperWalletError::OtpIsInvalid
        );

        Ok(())
    }
}
