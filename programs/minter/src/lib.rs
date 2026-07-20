pub mod constants;
pub mod error;
pub mod helper;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use helper::*;
pub use instructions::*;
pub use state::*;

declare_id!("3VkR6Q98H8tSCrC28GT8ePJcWYHkhvmM1rFo8BhcZc9E");

#[program]
pub mod minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        handle_initialize(ctx, args)
    }

    pub fn mint(ctx: Context<MintContext>, args: MintArgs) -> Result<()> {
        handle_mint(ctx, args)
    }

    pub fn forward(ctx: Context<Forward>, args: ForwardArgs) -> Result<()> {
        handle_forward(ctx, args)
    }
}
