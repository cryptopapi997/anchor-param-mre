use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("AG46nUNTxFqQDTpbVG5xWA7PkDiuZ7xccpRoghibUMXj");

// TODO: Replace with actual token mint
pub const TOKEN_MINT: Pubkey = Pubkey::new_from_array([
    160, 125, 200, 55, 211, 178, 66, 27, 149, 22, 219, 191, 28, 218, 171, 113, 92, 216, 236, 165,
    124, 20, 89, 205, 119, 106, 175, 166, 185, 155, 69, 242,
]);

#[program]
pub mod anchor_mre {

    use super::*;

    pub fn init_one(ctx: Context<InitializeOne>) -> Result<()> {
        ctx.accounts.thrd_acc.set_inner(ThrdAcc {
            bump: ctx.bumps.thrd_acc,
        });
        Ok(())
    }

    pub fn init_two(ctx: Context<InitializeTwo>) -> Result<()> {
        ctx.accounts.frth_acc.set_inner(FrthAcc {
            bump: ctx.bumps.frth_acc,
            buf: [0; 581],
            f5: None,
            f1: T4::default(),
            f4: None,
            f2: Vec::new(),
            f3: vec![],
        });

        ctx.accounts.fifth_acc.set_inner(FifthAcc {
            bump: ctx.bumps.fifth_acc,
            f2: vec![[0; 24]],
            f1: 0,
        });

        Ok(())
    }

    pub fn init_three(
        ctx: Context<InitializeThree>,
        _fst_acc_offset: u32,
        _snd_acc_offset: u32,
    ) -> Result<()> {
        ctx.accounts.snd_acc.set_inner(SndAcc {
            bump: ctx.bumps.snd_acc,
            buffer: [0; 107],
            f5: vec![],
            f6: vec![],
            f1: T2::default(),
            f3: vec![],
            f4: vec![],
            f2: false,
        });

        ctx.accounts.fst_acc.set_inner(FstAcc {
            bump: ctx.bumps.fst_acc,
            buffer: [0; 341],
            f3: vec![],
            f4: vec![],
            f1: None,
            f2: vec![],
        });
        Ok(())
    }

    pub fn demo(
        ctx: Context<Demo>,
        _fst_acc_offset: u32,
        _snd_acc_offset: u32,
        claim: MyParam,
    ) -> Result<()> {
        msg!("Buggy field is {:?}", claim.buggy_field);
        msg!(
            "We need this msg with a param or the bug doesn't happen {}",
            ctx.accounts.frth_acc.bump
        );

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_fst_acc_offset: u32,_snd_acc_offset: u32)]
pub struct InitializeThree<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 372,
        seeds = [b"FstAcc", _fst_acc_offset.to_le_bytes().as_ref()],
        bump
    )]
    pub fst_acc: Account<'info, FstAcc>,
    #[account(
        init,
        payer = signer,
        space = 202,
        seeds = [b"SndAcc", _snd_acc_offset.to_le_bytes().as_ref()],
        bump
    )]
    pub snd_acc: Account<'info, SndAcc>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTwo<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 648,
        seeds = [b"FrthAcc", signer.key().as_ref()],
        bump
    )]
    pub frth_acc: Box<Account<'info, FrthAcc>>,
    #[account(
        init,
        payer = signer,
        space = 17 + 24,
        seeds = [b"FifthAcc", frth_acc.key().as_ref()],
        bump
    )]
    pub fifth_acc: Box<Account<'info, FifthAcc>>,
    #[account(
        init,
        payer = signer,
        seeds = [b"Padding"],
        space = 24,
        bump
    )]
    pub padding: Box<Account<'info, Padding>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeOne<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"ThrdAcc"],
        space = 9,
        bump
    )]
    pub thrd_acc: Account<'info, ThrdAcc>,
    #[account(address = TOKEN_MINT)]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = thrd_acc,
    )]
    pub thrd_acc_ata: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(_fst_acc_offset: u32, _snd_acc_offset : u32)]
pub struct Demo<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Signer again
    pub signer_again: AccountInfo<'info>,
    #[account(mut,
        associated_token::mint = TOKEN_MINT,
        associated_token::authority = signer,
    )]
    pub signer_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"SndAcc", _snd_acc_offset.to_le_bytes().as_ref()],
        bump = snd_acc.bump,
        constraint = frth_acc.buf[0] == 0,
    )]
    pub snd_acc: Account<'info, SndAcc>,
    #[account(mut,
        seeds = [b"FstAcc", _fst_acc_offset.to_le_bytes().as_ref()],
        bump = fst_acc.bump
    )]
    pub fst_acc: Account<'info, FstAcc>,
    #[account(
        mut,
        seeds = [b"ThrdAcc"],
        bump = thrd_acc.bump
    )]
    pub thrd_acc: Account<'info, ThrdAcc>,
    #[account(mut,
        associated_token::mint = TOKEN_MINT,
        associated_token::authority = thrd_acc,
    )]
    pub thrd_acc_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"Padding"],
        bump
    )]
    pub padding: Account<'info, Padding>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        realloc = 17 + ((fifth_acc.f2.len() as i32 + frth_acc.f1.func2()) as usize) * 24,
        realloc::payer = signer,
        realloc::zero = false,
        seeds = [b"FifthAcc", frth_acc.key().as_ref()],
        bump = fifth_acc.bump
    )]
    pub fifth_acc: Box<Account<'info, FifthAcc>>,
    #[account(
        mut,
        realloc = frth_acc.func1(frth_acc.to_account_info().data_len())?,
        realloc::payer = signer,
        realloc::zero = false,
        seeds = [b"FrthAcc", signer.key().as_ref()],
        bump = frth_acc.bump
    )]
    pub frth_acc: Box<Account<'info, FrthAcc>>,
}

/// State

#[account]
pub struct FstAcc {
    pub f1: Option<T1>,
    pub buffer: [u8; 341],
    pub f2: Vec<u32>,
    pub f3: Vec<[u8; 13]>,
    pub f4: Vec<[u8; 13]>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct T1 {
    pub f1: Vec<u8>,
    pub f2: bool,
    pub f3: Vec<u32>,
}

#[account]
pub struct SndAcc {
    pub buffer: [u8; 107],
    pub f1: T2,
    pub f2: bool,
    pub f3: Vec<u32>,
    pub f4: Vec<u32>,
    pub f5: Vec<Pubkey>,
    pub f6: Vec<u8>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct T2 {
    pub f1: u32,
    pub f2: Option<T3>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct T3 {
    pub f1: [u8; 40],
    pub f2: String,
    pub f3: Vec<u8>,
    pub f4: Vec<u8>,
    pub f5: Vec<u8>,
    pub f6: Vec<u8>,
    pub f7: Vec<u8>,
}

#[account]
pub struct ThrdAcc {
    pub bump: u8,
}

#[account]
pub struct FrthAcc {
    pub buf: [u8; 581],
    pub f1: T4,
    pub f2: Vec<[u8; 48]>,
    pub f3: Vec<[u8; 24]>,
    pub f4: Option<u64>,
    pub f5: Option<Pubkey>,
    pub bump: u8,
}

impl FrthAcc {
    pub fn func1(&self, current_size: usize) -> Result<usize> {
        let _ = self.f1.func2();

        Ok(current_size)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct T4 {
    pub f1: [Vec<i64>; 2],
}

impl T4 {
    pub fn func2(&self) -> i32 {
        self.f1[0]
            .iter()
            .map(|x| if *x > 0 { 1 } else { -1 })
            .sum::<i32>()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MyParam {
    pub buf: [u8; 32],
    pub buggy_field: u128,
}

#[account]
pub struct FifthAcc {
    pub f1: u32,
    pub f2: Vec<[u8; 24]>,
    pub bump: u8,
}

#[account]
pub struct Padding {
    // [u8; 157]
    pub padding: Vec<u8>,
}
