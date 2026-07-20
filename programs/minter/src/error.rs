use anchor_lang::prelude::*;

#[error_code]
pub enum ContractError {
    #[msg("Wrong collection was provided for this mint")]
    WrongCollection,
    #[msg("Provided mint doesn't have collection")]
    NotInAnyCollection,
}
