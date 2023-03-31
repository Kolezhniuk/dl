use crate::wallet::{Wallet, Currency};



#[derive(Debug)]
struct UserApplication<'a> {
    wallet: &'a Wallet,
}

impl<'a> UserApplication<'a> {
    fn new(wallet: &'a Wallet) -> UserApplication<'a> {
        UserApplication { wallet }
    }

    fn print_balance(&self) {
        println!(
            "Balance: {} {}",
            match self.wallet.currency {
                Currency::Bitcoin => "BTC",
                Currency::Ethereum => "ETH",
            }
        );
    }
}
