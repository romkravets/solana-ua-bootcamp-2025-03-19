use anchor_lang::prelude::*;
use anchor_lang::system_program::ID as SystemID;

declare_id!("AiRCjmhxfkFcqjn7781ttJbtTKFvMGGHf81Y5A7GAZ5n");

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
        let favorites = &mut ctx.accounts.favorites;
        favorites.number = number;
        favorites.color = color;
        Ok(())
    }

    pub fn update_favorites(
        ctx: Context<UpdateFavorites>,
        number: Option<u64>, 
        color: Option<String>,
    ) -> Result<()> {
        let favorites = &mut ctx.accounts.favorites;

        if let Some(new_number) = number {
            favorites.number = new_number;
        }

        if let Some(new_color) = color {
            favorites.color = new_color;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 32 // Space for 'number' and 'color'
    )]
    pub favorites: Account<'info, Favorites>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFavorites<'info> {
    #[account(mut)] // Mark 'favorites' as mutable for updates
    pub favorites: Account<'info, Favorites>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Favorites {
    pub number: u64,      // Favorite number
    pub color: String,    // Favorite color
}






// use anchor_lang::prelude::*;

// declare_id!("AiRCjmhxfkFcqjn7781ttJbtTKFvMGGHf81Y5A7GAZ5n");

// pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// #[account]
// #[derive(InitSpace)]
// pub struct Favorites {
//     pub number: u64,

//     #[max_len(50)]
//     pub color: String,
// }

// // When people call the set_favorites instruction, they will need to provide the accounts that will
// // be modified. This keeps Solana fast!
// #[derive(Accounts)]
// pub struct SetFavorites<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,

//     #[account(
//         init,
//         payer = user,
//         space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
//         seeds = [b"favorites", user.key().as_ref()],
//         bump,
//     )]
//     pub favorites: Account<'info, Favorites>,

//     pub system_program: Program<'info, System>,
// }

// // Our Solana program!
// #[program]
// pub mod favorites {
//     use super::*;

//     // Our instruction handler! It sets the user's favorite number and color
//     pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
//         let user_public_key = context.accounts.user.key();
//         msg!("Greetings from {}", context.program_id);
//         msg!(
//             "User {}'s favorite number is {} and favorite color is: {}",
//             user_public_key,
//             number,
//             color
//         );

//         context
//             .accounts
//             .favorites
//             .set_inner(Favorites { number, color });
//         Ok(())
//     }

//     // We can also add a get_favorites instruction to get the user's favorite number and color
// }
