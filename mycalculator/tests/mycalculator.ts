import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Mycalculator } from "../target/types/mycalculator";
import { assert } from "chai";

describe("mycalculator", () => {
  // Set up the provider and program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Mycalculator as Program<Mycalculator>;

  // Generate a new keypair for the calculator account
  const calculatorKeypair = anchor.web3.Keypair.generate();
  
  // Create a new calculator account with a greeting 
  it("Creates a calculator account with a greeting", async () => {
    const tx = await program.methods
      .create("Hello, Solana!")
      .accounts({
        calculator: calculatorKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([calculatorKeypair])
      .rpc();

    console.log("Calculator initialized with transaction signature", tx);

    // Fetch the account to check stored data
    const account = await program.account.calculator.fetch(calculatorKeypair.publicKey);
    assert.equal(account.greeting, "Hello, Solana!", "Greeting should match");
  });
   
  //addition test case
  it("Performs an addition operation", async () => {
    const tx = await program.methods
      .add(new anchor.BN(10), new anchor.BN(20))
      .accounts({
        calculator: calculatorKeypair.publicKey,
      })
      .rpc();

    console.log("Addition performed with transaction signature", tx);

    // Fetch the updated calculator account
    const account = await program.account.calculator.fetch(calculatorKeypair.publicKey);
    assert.equal(account.result.toNumber(), 30, "Addition result should be 30");
  });


  //Subtraction test case
  it("Performs an subtraction operation", async () => {
    const tx = await program.methods
      .sub(new anchor.BN(30), new anchor.BN(20))
      .accounts({
        calculator: calculatorKeypair.publicKey,
      })
      .rpc();

    console.log("Subtraction performed with transaction signature", tx);

    // Fetch the updated calculator account
    const account = await program.account.calculator.fetch(calculatorKeypair.publicKey);
    assert.equal(account.result.toNumber(), 10, "Subtraction result should be 10");
  });


  //multiplication test-case
  it("Performs an multiplication operation", async () => {
    const tx = await program.methods
      .mul(new anchor.BN(5), new anchor.BN(10))
      .accounts({
        calculator: calculatorKeypair.publicKey,
      })
      .rpc();

    console.log("Multiplication performed with transaction signature", tx);

    // Fetch the updated calculator account
    const account = await program.account.calculator.fetch(calculatorKeypair.publicKey);
    assert.equal(account.result.toNumber(), 50, "Multiplication result should be 50");
  });

  //division test-case
  it("Performs an division operation", async () => {
    const tx = await program.methods
     .div(new anchor.BN(100), new anchor.BN(10))
     .accounts({
        calculator: calculatorKeypair.publicKey,
      })
     .rpc();

    console.log("Division performed with transaction signature", tx);

    // Fetch the updated calculator account
    const account = await program.account.calculator.fetch(calculatorKeypair.publicKey);
    assert.equal(account.result.toNumber(), 10, "Division result should be 10");
  });


});
