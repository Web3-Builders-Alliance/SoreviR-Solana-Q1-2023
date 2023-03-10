use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::MetadataAccount,
    token::{Mint, Token, TokenAccount, Transfer as SplTransfer},
};

declare_id!("7YKyo13HtdB823RiWHacDR74wc7VeU8vkMZGJDP2nSUB");

#[program]
pub mod deposit {

    use solana_program::hash;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        ctx.accounts.deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        Ok(())
    }

    //methods for depositing and withdrawing native tokens
    pub fn deposit_native(ctx: Context<DepositNative>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let sys_program = &ctx.accounts.system_program;

        deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();

        let cpi_accounts = system_program::Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn withdraw_native(ctx: Context<WithdrawNative>, amount: u64) -> Result<()> {
        let sys_program = &ctx.accounts.system_program;
        let deposit_account = &ctx.accounts.deposit_account;
        let pda_auth = &mut ctx.accounts.pda_auth;
        let sol_vault = &mut ctx.accounts.sol_vault;

        let cpi_accounts = system_program::Transfer {
            from: sol_vault.to_account_info(),
            to: ctx.accounts.deposit_auth.to_account_info(),
        };

        let seeds = &[
            b"sol_vault",
            pda_auth.to_account_info().key.as_ref(),
            &[deposit_account.sol_vault_bump.unwrap()],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(sys_program.to_account_info(), cpi_accounts, signer);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    //methods for depositing and withdrawing fungible SPL tokens
    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_acct.to_account_info(),
            to: ctx.accounts.to_token_acct.to_account_info(),
            authority: ctx.accounts.deposit_auth.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        anchor_spl::token::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
        let deposit_account = &ctx.accounts.deposit_account;

        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_acct.to_account_info(),
            to: ctx.accounts.to_token_acct.to_account_info(),
            authority: ctx.accounts.pda_auth.to_account_info(),
        };

        let seeds = &[
            b"auth",
            deposit_account.to_account_info().key.as_ref(),
            &[deposit_account.auth_bump],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        anchor_spl::token::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn create_limit(
        ctx: Context<CreateLimit>,
        ask_asset: Asset,
        ask_price_per_asset: u64,
    ) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = deposit_auth, space = DepositBase::LEN)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositNative<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawNative<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump = deposit_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/*
#[derive(Accounts)]
pub struct DepositSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        payer = deposit_auth,
        associated_token::authority = pda_auth,
    )]
    pub to_token_acct: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from_token_acct: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(mut)]
    pub to_token_acct: Account<'info, TokenAccount>,
    #[account(mut,
        associated_token::mint = token_mint,
        associated_token::authority = pda_auth,
    )]
    pub from_token_acct: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
*/
