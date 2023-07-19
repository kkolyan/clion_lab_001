use crate001::errors::ErrorChain;
use crate001::errors::ChainableResult;

pub fn main() {
    let result: Result<(), (i32, ErrorChain)> = Err((42, ErrorChain::new("root cause")));
    let result = result.chain_err(|| "secondary cause");
    result.unwrap_err().1.print_error();
}