# anchor-practise

To initialize new anchor project - anchor init <new-workspace-name>

1. The .anchor folder: It includes the most recent program logs and a local ledger that is used for testing
2. The app folder: An empty folder that you can use to hold your frontend if you use a monorepo
3. The programs folder: This folder contains your programs. It can contain multiple but initially only contains a program with the same name as <new-workspace-name>. This program already contains a lib.rs file with some sample code.
4. The tests folder: The folder that contains your E2E tests. It will already include a file that tests the sample code in the programs/<new-workspace-name>.
5. The migrations folder: In this folder you can save your deploy and migration scripts for your programs.
6. The Anchor.toml file: This file configures workspace wide settings for your programs. Initially, it configures
   The addresses of your programs on localnet ([programs.localnet])
   A registry your program can be pushed to ([registry])
   A provider which can be used in your tests ([provider])
   Scripts that Anchor executes for you ([scripts]). The test script is run when running anchor test. You can run your own scripts with anchor run <script_name>.

declare_id! macro defines the address of our program once it is deployed, either localnet, devnet, or mainnet. By default, it is filled with a dummy value that you should update with a real one once you are ready to deploy.

#[program]
pub mod hello_anchor {
use super::*;
pub fn initialize(ctx: Context) -> Result<()> {
ok(())
}
}

That’s our main entry point; there would be all of the logic we want to implement within our program. That’s how Anchor implements entrypoint.rs, instruction.rs, and processor.rs.

pub fn initialize(ctx: Context) -> Result<()> {
ok(())
}
Anchor requires a list of accounts that will be used by some instruction, that’s why each function declared under `#[program]` macro should have a required parameter of type `Context<T>`, that’s analog of `accounts: &[AccountInfo]`.
we can declare any amount of functions under `#[program]` macro, and all of them will be accessible to the client code.

#[derive(Accounts)]
pub struct Initialize {}
This instruction defines the context of initializing the entrypoint and is used when you are interested in interacting with some account data; in a simple case, it’s empty as we don’t need any account data for initialization for now. You may think about this as an Anchor approach to managing the state of the account that we have in `state.rs` file in the staking-app project.


An Anchor program consists of three parts:

👉 The program module marked with `#[program]` is where you define your business logic. You do so by writing functions that can be called by clients or other programs.  
👉 The Accounts structs which are marked with `#[derive(Accounts)]` is where you define which accounts your instruction expects and which constraints these accounts should adhere to. You do this via two constructs: Types and constraints.  
👉 The `declare_id!` macro creates an ID field that stores the address of your program. Anchor uses this hardcoded ID for security checks and allows other crates to access your program's address.  
