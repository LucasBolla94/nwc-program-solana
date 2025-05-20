use anchor_lang::prelude::*;
use crate::state::config::Config;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct UpdateRate<'info> {
    /// Conta de Configuracao com  o rate de outras infos
    #[account(mut, has_one = owner @ ErrorCode::Unauthorized)]
    pub config: Account<'info, Config>,

    /// CHECK: Onwer do programa ( Validacao ao Acesso )
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

pub fn handler(ctx: Context<UpdateRate>, new_rate: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;

    // Atualize o Rate com o novo Valor
    config.rate = new_rate;

    Ok(())
}