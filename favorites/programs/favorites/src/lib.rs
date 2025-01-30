use anchor_lang::prelude::*;

declare_id!("8Fj3FSmbs15BNM5UMa4fnwdA6FFZ44eDNNqfoqU7u3s5");

/// Anchor programs always allocate 8 bytes for the discriminator,
/// which uniquely identifies the account type
//In Anchor, every account associated with a program starts with an 8-byte discriminator that uniquely identifies the data structure. This ensures that the correct data type is being accessed
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

/// Define our Solana program using the Anchor framework
#[program]
pub mod favorites {
    use super::*;

    /// Instruction handler for setting the user's favorite number, color, and hobbies
    /// - `context`: Provides access to required accounts
    /// - `number`: A `u64` representing the user's favorite number
    /// - `color`: A `String` representing the user's favorite color
    /// - `hobbies`: A `Vec<String>` storing a list of hobbies
    pub fn set_favorites(
        context: Context<SetFavorites>, 
        number: u64, 
        color: String, 
        hobbies: Vec<String>
    ) -> Result<()> {
        
        /// Extract the public key of the user
        let user_public_key = context.accounts.user.key();
        
        /// Log messages to track execution
        msg!("Greetings from {}", context.program_id); //it is a Solana function that logs messages on-chain.
        msg!("User {user_public_key}'s favorite number is {number}, favorite color is: {color}");
        msg!("User's hobbies are: {:?}", hobbies);
        
        // Update the user's `favorites` account with the provided values
        context.accounts.favorites.set_inner(Favorites {
        //set_inner() replaces the old data in the PDA (Program Derived Account).
            number,
            color,
            hobbies
        });
        Ok(())
    }
}

/// Structure defining the data stored in the `Favorites` PDA (Program Derived Account)
#[account] //Marks it as solana accounts
#[derive(InitSpace)] //Automatically calculates account size for allocation.
pub struct Favorites {
    /// User's favorite number (64-bit unsigned integer)
    pub number: u64,

    /// User's favorite color (String with a max length of 50 characters)
    #[max_len(50)]
    pub color: String,

    /// User's hobbies (Vector containing up to 5 Strings, each max 50 characters long)
    #[max_len(5, 50)]
    pub hobbies: Vec<String>
}

/// Define the accounts required when calling `set_favorites`
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    
    /// The user signing the transaction (must be mutable because they pay fees)
    #[account(mut)]
    pub user: Signer<'info>,

    /// The user's unique `favorites` account, storing their number, color, and hobbies
    /// - `init_if_needed`: Initializes the account if it doesn't already exist
    /// - `payer = user`: The user funds the creation of this account
    /// - `space`: Specifies the allocated size (discriminator + computed struct size)
    /// - `seeds`: Defines a unique PDA using "favorites" + user public key
    /// - `bump`: Provides a bump seed for security
    #[account(
        init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, 
        seeds=[b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    /// Solana's built-in System Program, required for account creation and interaction
    pub system_program: Program<'info, System>,
}