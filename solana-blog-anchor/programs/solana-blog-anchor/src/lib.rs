use anchor_lang::prelude::*;

declare_id!("bhB5FcRMygdF3FS65442s7pfmyG3YcSYxEVUYiKMHis");

#[program]
pub mod solana_blog_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
