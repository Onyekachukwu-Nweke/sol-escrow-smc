use anchor_lang::prelude::*;

declare_id!("2jDJbiWwoGCi2k11h172zBiD657y1uByM8g92NRNLa4X");

#[program]
pub mod sol_escrow_smc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
