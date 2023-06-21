#![allow(unused)]
fn main() {
    // counter data structure
    pub struct counter {
        pub count: u64,
        pub is_initialized: bool,
    }

    fn increment(accounts) {
        let counter = deserialize(accounts.counter);
        counter.count += 1;
    }
    struct counterCounter {
        count: u64,
    }
}

// program composition via multiple instructions in a transaction
// initialize function (pseudo code)
fn initialize(accounts) {
    let counter = deserialize(accounts.counter);
    if counter.is_initialized {
        error("Already initialized");
    }
    counter.count = 0;
    counter.is_initialized = true;
}

// program composition via Cross Program Invocation (CPI)
fn initialize(accounts) {
    accounts.system_program.create_account(accounts.payer, accounts.counter);
    let counter = deserialize(accounts.counter);
    counter.count = 0;
}

// validate program inputs
fn transfer(accounts, lamports) {
    if !accounts.from.is_signer {
        error();
    }
    accounts.from.lamports -= lamports;
    accounts.to.lamports += lamports;
}