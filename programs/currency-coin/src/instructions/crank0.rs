use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn crank0(
    ctx: Context<Crank0>,
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccs0_mint_bump: u8,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.maturity_state == 0
        || ctx.accounts.mint_authority.maturity_state == 2, true);
    let ir = ctx.accounts.mint_authority.ccb_amount
        / ctx.accounts.mint_authority.cc0_amount
        * ctx.accounts.mint_authority.imod;
    let mut cc_to_mint = (ir * ctx.accounts.mint_authority.cc0_amount).floor();
    if cc_to_mint < 1.0 { cc_to_mint = 1.0; }
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
        ), cc_to_mint as u64,
    )?;

    let s0_to_mint = (ir * ctx.accounts.mint_authority.ccs_amount).floor();
    if s0_to_mint > 0.0 {
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
            ), s0_to_mint as u64,
        )?;
    }
    let pool0 = &mut ctx.accounts.mint_authority;
    pool0.imod *= (pool0.cc0_amount + cc_to_mint) / pool0.cc0_amount;

    let clock: Clock = Clock::get().unwrap();
    let dt = clock.unix_timestamp - pool0.timestamp;
    assert_eq!(dt >= 5, true);
    pool0.timestamp = clock.unix_timestamp;
    let ips: f32 = (ir / pool0.cc0_amount / dt as f64) as f32;
    pool0.ima0 *= 100.0;
    pool0.ima0 += ips;
    pool0.ima0 /= 101.0;

    pool0.ima1 *= 10000.0;
    pool0.ima1 += ips;
    pool0.ima1 /= 10001.0;

    pool0.ima2 *= 1000000.0;
    pool0.ima2 += ips;
    pool0.ima2 /= 1000001.0;

    pool0.cc0_amount += cc_to_mint;
    pool0.ccs_amount += s0_to_mint;

    if pool0.cc0_amount >= 1.01 * pool0.ccb_amount {
        if pool0.maturity_state == 0 { pool0.maturity_state = 1; }
        if pool0.maturity_state == 2 { pool0.maturity_state = 3; }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccs0_mint_bump: u8,
)]
pub struct Crank0<'info> {
    #[account(mut,
        seeds = [ b"mint_auth_" ],
        bump = mint_auth_bump
    )]
    pub mint_authority: Account<'info, MintAuth>,
    #[account(mut,
        seeds = [ b"cc_mint_" ],
        bump = cc_mint_bump
    )]
    pub cc_mint_account: Account<'info, token::Mint>,
    #[account(mut,
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
