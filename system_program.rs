/// simplified system program code

fn transfer(accounts, lamports) {
    if !accounts.from.is_signer {
        error();
    }
    accounts.from.lamports -= lamports;
    accounts.to.lamports += lamports;
}

/// A program may not write to any accounts that it doesn't own
/// Other than transferring lamports, the system program is used to create 
/// accounts for other programs.
/// An account is created with a specific size and a specific amount of 
/// lamports.