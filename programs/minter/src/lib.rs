pub mod constants;

pub mod error;
pub mod helper;

use anchor_lang::prelude::*;

pub use constants::*;

pub use helper::*;

declare_id!("3VkR6Q98H8tSCrC28GT8ePJcWYHkhvmM1rFo8BhcZc9E");

#[derive(Accounts)]
pub struct HelloWorld {}

#[program]
pub mod minter {
    use super::*;

    pub fn hello_world(ctx: Context<HelloWorld>) -> Result<()> {
        Ok(())
    }
}
