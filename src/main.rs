mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<types::AccountId,types::BlockNumber,types::Nonce>,
	balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {

        Self { system: system::Pallet::new(), balances: balances::Pallet::new() }

    }
}

fn main() {
    /* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	let mut runtime = Runtime::new();
    /* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
	let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    // start emulating a block
    /* TODO: Increment the block number in system. */
	runtime.balances.set_balance(&alice, 100);
    /* TODO: Assert the block number is what we expect. */
	runtime.system.inc_block_number();
	let bn = runtime.system.block_number(); // read the updated value
	println!("Block number: {}", bn);
	assert_eq!(runtime.system.block_number(),1);

    // first transaction
    /* TODO: Increment the nonce of `alice`. */
	runtime.system.inc_nonce(&alice);
    /* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
        - The transfer _could_ return an error. We should use `map_err` to print
          the error if there is one.
        - We should capture the result of the transfer in an unused variable like `_res`.
    */
	let _res = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e| eprintln!("{}", e));

    // second transaction
    /* TODO: Increment the nonce of `alice` again. */
	runtime.system.inc_nonce(&alice);
    /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
	let _res = runtime.balances.transfer(alice.clone(), charlie, 20);

	println!("{:#?}", runtime);
}
