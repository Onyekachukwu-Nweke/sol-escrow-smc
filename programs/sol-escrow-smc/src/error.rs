use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid escrow state for this operation")]
    InvalidEscrowState,
    
    #[msg("Unauthorized depositor - you are not a party to this escrow")]
    UnauthorizedDepositor,
    
    #[msg("Party has already deposited their tokens")]
    AlreadyDeposited,
    
    #[msg("Escrow has expired and is no longer valid")]
    EscrowExpired,
    
    #[msg("Invalid party for this escrow")]
    InvalidParty,
    
    #[msg("Cannot cancel escrow yet - conditions not met")]
    CannotCancelYet,
    
    #[msg("Insufficient token balance for deposit")]
    InsufficientBalance,
    
    #[msg("Token mint does not match escrow requirements")]
    InvalidTokenMint,
    
    #[msg("Escrow is currently paused")]
    EscrowPaused,
    
    #[msg("Invalid timeout duration - must be positive")]
    InvalidTimeout,
    
    #[msg("Escrow ID already exists")]
    DuplicateEscrowId,
}