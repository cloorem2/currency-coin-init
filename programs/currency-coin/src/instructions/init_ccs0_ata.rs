use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};
use crate::create_mint_auth::MintAuth;

pub fn init_ccs0_ata(
    ctx: Context<InitCcs0Ata>,
    mint_auth_bump: u8,
    _mint_bump: u8,
) -> Result<()> {
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ),
        0,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(mint_auth_bump: u8, mint_bump: u8)]
pub struct InitCcs0Ata<'info> {
    #[account(mut,
        seeds = [ b"mint_auth_" ],
        bump = mint_auth_bump
    )]
    pub mint_authority: Account<'info, MintAuth>,
    #[account(mut,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
        seeds = [ b"ccs0_mint_" ],
        bump = mint_bump
    )]
    pub mint_account: Account<'info, token::Mint>,
    #[account(init,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = mint_authority,
    )]
    pub token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
