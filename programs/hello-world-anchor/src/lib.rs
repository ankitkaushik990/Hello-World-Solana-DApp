use anchor_lang::prelude::*;

declare_id!("6vbGtfc9161doANhVPUPHntJDXrkEjyoXLaXQNxnTwdB");

#[program]
pub mod hello_world_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
