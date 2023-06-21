#![allow(unused)]
fn main() {
    // counter data structure
    pub struct counter {
        pub count: u64,
        pub is_initialized: bool,
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
