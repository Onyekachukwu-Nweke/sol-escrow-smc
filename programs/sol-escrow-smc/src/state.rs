use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    /// Unique identifier for this escrow
    pub escrow_id: u64,
    /// Public key of party A (initiator)
    pub party_a: Pubkey,
    /// Public key of party B (counterparty)
    pub party_b: Pubkey,
    /// Mint address of token A
    pub token_a_mint: Pubkey,
    /// Mint address of token B
    pub token_b_mint: Pubkey,
    /// Amount of token A to be exchanged
    pub token_a_amount: u64,
    /// Amount of token B to be exchanged
    pub token_b_amount: u64,
    /// Current state of the escrow
    pub state: EscrowState,
    /// Timestamp when escrow was created
    pub created_at: i64,
    /// Timestamp when escrow expires
    pub timeout_at: i64,
    /// Timestamp when swap was completed (if applicable)
    pub completed_at: Option<i64>,
    /// Whether party A has deposited their tokens
    pub party_a_deposited: bool,
    /// Whether party B has deposited their tokens
    pub party_b_deposited: bool,
    /// PDA bump seed
    pub bump: u8,
}

impl EscrowAccount {
    /// Calculate the space needed for this account
    pub const ACCOUNT_SIZE: usize = 8 + // Discriminator
        8 + // escrow_id
        32 + // party_a
        32 + // party_b
        32 + // token_a_mint
        32 + // token_b_mint
        8 + // token_a_amount
        8 + // token_b_amount
        1 + // state (enum)
        8 + // created_at
        8 + // timeout_at
        9 + // completed_at (Option<i64>)
        1 + // party_a_deposited
        1 + // party_b_deposited
        1; // bump

    /// Check if both parties have deposited
    pub fn both_deposited(&self) -> bool {
        self.party_a_deposited && self.party_b_deposited
    }

    /// Check if escrow has expired
    pub fn is_expired(&self, current_timestamp: i64) -> bool {
        current_timestamp >= self.timeout_at
    }

    /// Check if escrow can be cancelled
    pub fn can_cancel(&self, current_timestamp: i64) -> bool {
        self.is_expired(current_timestamp) || self.state != EscrowState::BothDeposited
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum EscrowState {
    /// Escrow created, waiting for deposits
    Initialized,
    /// One party has deposited, waiting for the other
    PartialDeposit,
    /// Both parties have deposited, ready for swap
    BothDeposited,
    /// Swap has been executed successfully
    Completed,
    /// Escrow was cancelled and funds refunded
    Cancelled,
    /// Escrow is paused by governance
    Paused,
}