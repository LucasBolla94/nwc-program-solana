use anchor_lang::prelude::*;
use crate::state::config::Config;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct SetPaused<'info> {
    /// Conta de configuracao principal do programa
    #[account(mut, has_one = owner @ ErrorCode::Unauthorized)]
    pub config: Account<'info, Config>,

    /// CHECK: Owner verificado por has_one e precisa ser Signer
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

/// Permite o Owner pausar ou despausar o programa
pub fn pause_handler(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config; 

    // Atualiza a flag de pausa
    config.paused = paused; 

    Ok(())
}