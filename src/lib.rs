use anchor_lang::prelude::*;

declare_id!("95UyNHWjir7Jyjr4hSr7rCRZWxq9dfy6NGfbtmuFPvha");

#[program]
pub mod c8ntinuum {
    use super::*;

    // The context parameter must be present; even if you don’t need any accounts.
    pub fn test(ctx: Context<Empty>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Empty {}