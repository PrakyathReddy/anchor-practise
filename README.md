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
