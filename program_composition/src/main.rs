#![allow(unused)]
fn main() {
    // counter data structure
    pub struct counter {
        pub count: u64,
        pub is_initialized: bool,
    }
}

// initialize function (pseudo code)
fn initialize(accounts) {
    let counter = deserialize(accounts.counter);
    if counter.is_initialized {
        error("Already initialized");
    }
    counter.count = 0;
    counter.is_initialized = true;
}