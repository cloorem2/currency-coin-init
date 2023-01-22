use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn init_pool0(
    ctx: Context<InitPool0>,
    mint_auth_bump: u8,
    _cc_mint_bump: u8,
    _ccb0_mint_bump: u8,
    _ccs0_mint_bump: u8,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.imod,0.0);
    let x0: u64 = 1;
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.cc_mint_account.to_account_info(),
                to: ctx.accounts.cc_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), 2 * x0,
    )?;
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.ccb0_mint_account.to_account_info(),
                to: ctx.accounts.ccb0_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), x0,
    )?;
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.ccs0_mint_account.to_account_info(),
                to: ctx.accounts.ccs0_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), x0,
    )?;
    let pool0 = &mut ctx.accounts.mint_authority;
    pool0.imod = 0.1;
    pool0.smod = 1.0;
    pool0.rmod = 1.0;
    pool0.cmod = 1.0;
    pool0.isum = 1.0;
    let clock: Clock = Clock::get().unwrap();
    pool0.timestamp = clock.unix_timestamp;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccs0_mint_bump: u8,
)]
pub struct InitPool0<'info> {
    #[account(mut,
        seeds = [ b"mint_auth_" ],
        bump = mint_auth_bump
    )]
    pub mint_authority: Account<'info, MintAuth>,
    #[account(mut,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
        seeds = [ b"cc_mint_" ],
        bump = cc_mint_bump
    )]
    pub cc_mint_account: Account<'info, token::Mint>,
    #[account(mut,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
        seeds = [ b"ccb0_mint_" ],
        bump = ccb0_mint_bump
    )]
    pub ccb0_mint_account: Account<'info, token::Mint>,
    #[account(mut,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
        seeds = [ b"ccs0_mint_" ],
        bump = ccs0_mint_bump
    )]
    pub ccs0_mint_account: Account<'info, token::Mint>,
    #[account(mut,
        associated_token::mint = cc_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub cc_token_account: Account<'info, token::TokenAccount>,
    #[account(mut,
        associated_token::mint = ccb0_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccb0_token_account: Account<'info, token::TokenAccount>,
    #[account(mut,
        associated_token::mint = ccs0_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccs0_token_account: Account<'info, token::TokenAccount>,
    // #[account(mut)]
    // pub payer: Signer<'info>,
    // pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    // pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
