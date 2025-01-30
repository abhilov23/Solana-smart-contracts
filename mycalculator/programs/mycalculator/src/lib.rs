use anchor_lang::prelude::*;



declare_id!("A9RA4yLnVJyqxYMpPWVcqYBgRjQfZSRNbHzE23c7K3DA");


#[error_code]
pub enum MyError {
    #[msg("Error: Division by zero is not allowed.")]
    DivisionByZero,
}


#[program]
pub mod mycalculator {
    use super::*;

    pub fn create(ctx:Context<Create>, init_message: String) -> Result<()>{
        let calculator = &mut ctx.accounts.calculator; //picking up the context from the data account
        calculator.greeting = init_message;
        Ok(())
    }
     
     pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> Result<()>{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
     }

    //subtraction function
    pub fn sub(ctx: Context<Subtraction>, num1: i64, num2: i64) -> Result<()>{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    //multiplication function
    pub fn mul(ctx: Context<Multiplication>, num1: i64, num2: i64) -> Result<()>{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }
    //division function
    pub fn div(ctx: Context<Division>, num1: i64, num2: i64) -> Result<()>{
        let calculator = &mut ctx.accounts.calculator;
        if num2!= 0 {
            calculator.result = num1 / num2;

        } 
        else {
            msg!("Error: Division by zero"); // Log the error
            return Err(MyError::DivisionByZero.into());
        }
        Ok(())
    }

}



#[derive(Accounts)]
pub struct Create<'info> { //first account : will not store on-chain, It's a struct that represents the context for an instruction execution.
    #[account(init,payer=user,space=264)]
   pub calculator: Account<'info, Calculator>,
   #[account(mut)]
   pub user: Signer<'info>,
   pub system_program: Program<'info, System>,
}

 //addition context
#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
   pub calculator: Account<'info, Calculator>,
}
 //subtract context
 #[derive(Accounts)]
 pub struct Subtraction<'info> {
    #[account(mut)]
   pub calculator: Account<'info, Calculator>,
 }

 //multiplication context
 #[derive(Accounts)]
 pub struct Multiplication<'info> {
    #[account(mut)]
   pub calculator: Account<'info, Calculator>,
 }

 //division context
 #[derive(Accounts)]
 pub struct Division<'info> {
    #[account(mut)]
   pub calculator: Account<'info, Calculator>,
 }




#[account]
pub struct Calculator { //2nd account : will store data on-onchain
    pub greeting: String,
    pub result:i64,
    pub remainder: i64,
}