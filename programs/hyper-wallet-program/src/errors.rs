use anchor_lang::error_code;

#[error_code]
pub enum HyperWalletError {
    GeneralError,
    OtpIsRequired,
    OtpProofPathIsRequired,
    OtpIsInvalid,
    IllegalHyperWalletOwner,
    AddressAlreadyWhiteListed,
    AddressNotWhiteListed,
}
