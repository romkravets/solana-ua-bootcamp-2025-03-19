// use anchor_lang::prelude::*;

// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
// };

// use crate::{Offer, ANCHOR_DISCRIMINATOR};

// #[derive(Accounts)]
// #[instruction(id: u64)]
// pub struct MakeOffer<'info> {
//     #[account(mut)]
//     pub maker: Signer<'info>,

//     #[account(mint::token_program = token_program)]
//     pub token_mint_a: InterfaceAccount<'info, Mint>,

//     #[account(mint::token_program = token_program)]
//     pub token_mint_b: InterfaceAccount<'info, Mint>,

//     #[account(
//         mut,
//         associated_token::mint = token_mint_a,
//         associated_token::authority = maker,
//         associated_token::token_program = token_program
//     )]
//     pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

//     #[account(
//         init,
//         payer = maker,
//         space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
//         seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub offer: Account<'info, Offer>,

//     #[account(
//         init,
//         payer = maker,
//         associated_token::mint = token_mint_a,
//         associated_token::authority = offer,
//         associated_token::token_program = token_program
//     )]
//     pub vault: InterfaceAccount<'info, TokenAccount>,

//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub token_program: Interface<'info, TokenInterface>,
//     pub system_program: Program<'info, System>,
// }

// pub fn send_offered_tokens_to_vault(
//     context: &Context<MakeOffer>,
//     token_a_offered_amount: u64,
// ) -> Result<()> {
//     let transfer_accounts = TransferChecked {
//         from: context.accounts.maker_token_account_a.to_account_info(),
//         mint: context.accounts.token_mint_a.to_account_info(),
//         to: context.accounts.vault.to_account_info(),
//         authority: context.accounts.maker.to_account_info(),
//     };

//     let cpi_context = CpiContext::new(
//         context.accounts.token_program.to_account_info(),
//         transfer_accounts,
//     );

//     transfer_checked(
//         cpi_context,
//         token_a_offered_amount,
//         context.accounts.token_mint_a.decimals,
//     )
// }

// pub fn save_offer(context: Context<MakeOffer>, id: u64, token_b_wanted_amount: u64) -> Result<()> {
//     context.accounts.offer.set_inner(Offer {
//         id,
//         maker: context.accounts.maker.key(),
//         token_mint_a: context.accounts.token_mint_a.key(),
//         token_mint_b: context.accounts.token_mint_b.key(),
//         token_b_wanted_amount,
//         bump: context.bumps.offer,
//     });
//     Ok(())
// }

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn create_offer(
    ctx: Context<CreateOffer>,
    id: u64,
    token_a_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // Save offer data
    ctx.accounts.offer.set_inner(Offer {
        id,
        maker: ctx.accounts.maker.key(),
        token_mint_a: ctx.accounts.token_mint_a.key(),
        token_mint_b: ctx.accounts.token_mint_b.key(),
        token_a_amount,
        token_b_wanted_amount,
        bump: ctx.bumps.offer,
    });

    // Approve the offer account to transfer token_a_amount from maker
    let approve_ix = spl_token::instruction::approve_checked(
        &ctx.accounts.token_program.key(),
        &ctx.accounts.maker_token_account_a.key(),
        &ctx.accounts.token_mint_a.key(),
        &ctx.accounts.offer.key(),
        &ctx.accounts.maker.key(),
        &[],
        token_a_amount,
        ctx.accounts.token_mint_a.decimals,
    )?;
    anchor_lang::solana_program::program::invoke(
        &approve_ix,
        &[
            ctx.accounts.maker_token_account_a.to_account_info(),
            ctx.accounts.token_mint_a.to_account_info(),
            ctx.accounts.offer.to_account_info(),
            ctx.accounts.maker.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
    )?;

    Ok(())
}



