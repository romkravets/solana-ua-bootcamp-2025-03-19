// pub mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;

// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("81nN5ZUyyu7h3XCXaGRLTmXsWDW6B4mAnYAJFHQp2die");

// #[program]
// pub mod escrow {
//     use super::*;

//     pub fn make_offer(
//         context: Context<MakeOffer>,
//         id: u64,
//         token_a_offered_amount: u64,
//         token_b_wanted_amount: u64,
//     ) -> Result<()> {
//         instructions::make_offer::send_offered_tokens_to_vault(&context, token_a_offered_amount)?;
//         instructions::make_offer::save_offer(context, id, token_b_wanted_amount)
//     }

//     pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
//         instructions::take_offer::send_wanted_tokens_to_maker(&context)?;
//         instructions::take_offer::withdraw_and_close_vault(context)
//     }
    
// }

use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const ANCHOR_DISCRIMINATOR: usize = 8;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use constants::*;
use instructions::*;
use state::*;

declare_id!("81nN5ZUyyu7h3XCXaGRLTmXsWDW6B4mAnYAJFHQp2die");

#[program]
pub mod escrow {
    use super::*;

    /// Створення оферу:
    /// - переводимо токени `token_a` в сейф (vault)
    /// - зберігаємо стан оферу
    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        // 1. Переводимо запропоновані токени в vault
        instructions::make_offer::send_offered_tokens_to_vault(&context, token_a_offered_amount)?;
        
        // 2. Зберігаємо деталі оферу
        instructions::make_offer::save_offer(context, id, token_b_wanted_amount)
    }

    /// Прийняття оферу:
    /// - переводимо токени `token_b` від тейкера до мейкера
    /// - переводимо токени `token_a` з vault в акаунт тейкера
    /// - закриваємо vault акаунт
    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        // 1. Відправляємо бажану кількість `token_b` мейкеру
        instructions::take_offer::send_wanted_tokens_to_maker(&context)?;
        
        // 2. Виводимо `token_a` з сейфу до тейкера і закриваємо сейф
        instructions::take_offer::withdraw_and_close_vault(context)
    }
}


