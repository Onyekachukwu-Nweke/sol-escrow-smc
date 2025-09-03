use anchor_lang::prelude::*;

declare_id!("2jDJbiWwoGCi2k11h172zBiD657y1uByM8g92NRNLa4X");

#[program]
pub mod sol_escrow_smc {
    use super::*;

    /// Initialize a new escrow
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        escrow_id: u64,
        token_a_amount: u64,
        token_b_amount: u64,
        timeout_duration: i64,
    ) -> Result<()> {
        Ok(())
    }
}