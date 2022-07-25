use anchor_lang::prelude::*;

#[error_code]
pub enum TarsError {
    #[msg("Account does not have correct owner!")]
    IncorrectOwner,
    #[msg("Account is not initialized!")]
    Uninitialized,
    #[msg("Mint Mismatch!")]
    MintMismatch,
    #[msg("Index greater than length!")]
    IndexGreaterThanLength,
    #[msg("Numerical overflow error!")]
    NumericalOverflowError,
    #[msg("Can only provide up to 4 creators to tars (because tars is one)!")]
    TooManyCreators,
    #[msg("Uuid must be exactly of 6 length")]
    UuidMustBeExactly6Length,
    #[msg("Not enough tokens to pay for this minting")]
    NotEnoughTokens,
    #[msg("Not enough SOL to pay for this minting")]
    NotEnoughSOL,
    #[msg("Token transfer failed")]
    TokenTransferFailed,
    #[msg("Tars is empty!")]
    TarsEmpty,
    #[msg("Tars is not live!")]
    TarsNotLive,
    #[msg("Configs that are using hidden uris do not have config lines, they have a single hash representing hashed order")]
    HiddenSettingsConfigsDoNotHaveConfigLines,
    #[msg("Cannot change number of lines unless is a hidden config")]
    CannotChangeNumberOfLines,
    #[msg("Derived key invalid")]
    DerivedKeyInvalid,
    #[msg("Public key mismatch")]
    PublicKeyMismatch,
    #[msg("No whitelist token present")]
    NoWhitelistToken,
    #[msg("Token burn failed")]
    TokenBurnFailed,
    #[msg("Missing gateway app when required")]
    GatewayAppMissing,
    #[msg("Missing gateway token when required")]
    GatewayTokenMissing,
    #[msg("Invalid gateway token expire time")]
    GatewayTokenExpireTimeInvalid,
    #[msg("Missing gateway network expire feature when required")]
    NetworkExpireFeatureMissing,
    #[msg("Unable to find an unused config line near your random number index")]
    CannotFindUsableConfigLine,
    #[msg("Invalid string")]
    InvalidString,
    #[msg("Suspicious transaction detected")]
    SuspiciousTransaction,
    #[msg("Cannot Switch to Hidden Settings after items available is greater than 0")]
    CannotSwitchToHiddenSettings,
    #[msg("Incorrect SlotHashes PubKey")]
    IncorrectSlotHashesPubkey,
    #[msg("Incorrect collection NFT authority")]
    IncorrectCollectionAuthority,
    #[msg("Collection PDA address is invalid")]
    MismatchedCollectionPDA,
    #[msg("Provided mint account doesn't match collection PDA mint")]
    MismatchedCollectionMint,
    #[msg("Slot hashes Sysvar is empty")]
    SlotHashesEmpty,
    #[msg("The metadata account has data in it, and this must be empty to mint a new NFT")]
    MetadataAccountMustBeEmpty,
    #[msg("Missing set collection during mint IX for Tars with collection set")]
    MissingSetCollectionDuringMint,
    #[msg("Can't change collection settings after items have begun to be minted")]
    NoChangingCollectionDuringMint,
    #[msg("Retain authority must be true for Tars with a collection set")]
    TarsCollectionRequiresRetainAuthority,
    #[msg("Error within Gateway program")]
    GatewayProgramError,
    #[msg("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance,
    #[msg("Can't decerease as the count is more than number of available spots.")]
    InvalidNumberofWL,
    #[msg("WLType is invalid.")]
    InvalidWLType,
    #[msg("WL1 not scheduled.")]
    WL1NotScheduled,
    #[msg("WL2 not scheduled.")]
    WL2NotScheduled,
    #[msg("WL3 not scheduled.")]
    WL3NotScheduled,
    #[msg("WL4 not scheduled.")]
    WL4NotScheduled,
    #[msg("WL mint not started yet.")]
    WLMintNotStarted,
    #[msg("No whitelist spots left")]
    NoWhitelistSpots,
    #[msg("Magic hat is not live for WL!")]
    TarsNotLiveForWL,
}

