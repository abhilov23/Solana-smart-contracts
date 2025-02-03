use anchor_lang::prelude::*;

declare_id!("8Fj3FSmbs15BNM5UMa4fnwdA6FFZ44eDNNqfoqU7u3s5");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        context: Context<SetFavorites>, 
        number: u64, 
        color: String, 
        hobbies: Vec<String>
    ) -> Result<()> {
        
        let bump = *context.bumps.get("favorites").unwrap(); // Extract bump
        
        require!(color.len() <= 50, CustomError::ColorTooLong);
        require!(hobbies.len() <= 5, CustomError::TooManyHobbies);
        for hobby in &hobbies {
            require!(hobby.len() <= 50, CustomError::HobbyTooLong);
        }

        msg!("Favorites updated successfully for user.");

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)] 
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, 
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    #[msg("Color is too long! Max 50 characters.")]
    ColorTooLong,
    #[msg("Too many hobbies! Max 5 hobbies allowed.")]
    TooManyHobbies,
    #[msg("A hobby is too long! Max 50 characters per hobby.")]
    HobbyTooLong,
}
