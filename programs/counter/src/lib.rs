use anchor_lang::prelude::*;

declare_id!("9vKyKVGXNgJyygnHcAg8hKLZZnHEteV83b6e56qScyTc");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
