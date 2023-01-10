use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn buy_bonds0(
    ctx: Context<BuyBonds0>,
    // the most cc buyer is sending, we leave the dust behind
    amount: u64,
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccs0_mint_bump: u8,
    // ccb0_mint_account: Pubkey,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.maturity_state, 0);
    // let r0: u64 = ctx.accounts.mint_authority.cc0_amount;
    // let r1: u64 = ctx.accounts.mint_authority.ccb_amount;
    // let k: u64 = r0 * r1;
    // let r00: u64 = r0 + amount;
    // let ccb_amount: u64 = (r1 * r00 - k) / r00;
    // let r11: u64 = r1 - ccb_amount;
    // let cc_amount: u64 = (k - r0 * r11) / r11;

    let mut bonds_to_owner = amount as f64;
    bonds_to_owner += ctx.accounts.mint_authority.cc0_amount;
    bonds_to_owner *= ctx.accounts.mint_authority.ccb_amount;
    bonds_to_owner -= ctx.accounts.mint_authority.cc0_amount
      * ctx.accounts.mint_authority.ccb_amount;
    bonds_to_owner /= ctx.accounts.mint_authority.cc0_amount + amount as f64;
    bonds_to_owner = bonds_to_owner.floor();
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.ccb0_account.to_account_info(),
                to: ctx.accounts.owner_ccb0_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), bonds_to_owner as u64,
    )?;

    let mut cc_from_owner = ctx.accounts.mint_authority.ccb_amount
      - bonds_to_owner;
    cc_from_owner *= ctx.accounts.mint_authority.cc0_amount;
    cc_from_owner = ctx.accounts.mint_authority.cc0_amount
      * ctx.accounts.mint_authority.ccb_amount - cc_from_owner;
    cc_from_owner /= ctx.accounts.mint_authority.ccb_amount - bonds_to_owner;
    cc_from_owner = cc_from_owner.ceil();
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_cc_account.to_account_info(),
                to: ctx.accounts.cc_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ), cc_from_owner as u64,
    )?;

    let mut s0_to_mint = cc_from_owner;
    s0_to_mint *= ctx.accounts.mint_authority.ccs_amount;
    s0_to_mint /= ctx.accounts.mint_authority.cc0_amount;
    s0_to_mint = s0_to_mint.ceil();
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.ccs0_mint_account.to_account_info(),
                to: ctx.accounts.ccs0_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), s0_to_mint as u64,
    )?;

    let mut cc_to_burn = s0_to_mint;
    cc_to_burn += ctx.accounts.mint_authority.ccs_amount;
    cc_to_burn *= ctx.accounts.mint_authority.cc1_amount;
    cc_to_burn -= ctx.accounts.mint_authority.cc1_amount
      * ctx.accounts.mint_authority.ccs_amount;
    cc_to_burn /= ctx.accounts.mint_authority.ccs_amount + s0_to_mint;
    cc_to_burn = cc_to_burn.floor();
    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.cc_mint_account.to_account_info(),
                from: ctx.accounts.cc_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), cc_to_burn as u64,
    )?;

    ctx.accounts.mint_authority.cc0_amount += cc_from_owner;
    ctx.accounts.mint_authority.ccb_amount -= bonds_to_owner;
    ctx.accounts.mint_authority.cc1_amount -= cc_to_burn;
    ctx.accounts.mint_authority.ccs_amount += s0_to_mint;
    Ok(())
}


#[derive(Accounts)]
#[instruction(
    amount: u64,
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccs0_mint_bump: u8,
    // cc_mint_account: Pubkey,
    // ccb0_mint_account: Pubkey,
)]
pub struct BuyBonds0<'info> {
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
        seeds = [ b"ccb0_mint_" ],
        bump = ccb0_mint_bump
    )]
    pub ccb0_mint_account: Account<'info, token::Mint>,
    #[account(mut,
        seeds = [ b"ccs0_mint_" ],
        bump = ccs0_mint_bump
    )]
    pub ccs0_mint_account: Account<'info, token::Mint>,

    #[account(mut,
        associated_token::mint = cc_mint_account,
        associated_token::authority = owner,
    )]
    pub owner_cc_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb0_mint_account,
        associated_token::authority = owner,
    )]
    pub owner_ccb0_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = cc_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub cc_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb0_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccb0_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccs0_mint_account,
        associated_token::authority = mint_authority,
    )]
    pub ccs0_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    // pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    // pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
