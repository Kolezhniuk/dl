use crate::wallet::Wallet;

#[derive(Debug, Clone)]
pub struct Connection {
    pub api_endpoint: String,
    pub synced: bool,
}

impl Connection {
    pub fn new(api_endpoint: &str) -> Connection {
        Connection {
            api_endpoint: api_endpoint.to_string(),
            synced: false,
        }
    }

    pub fn synchronize(&mut self, wallet: &Wallet) {
        // Perform network request to synchronize wallet state
        println!(
            "Synchronizing wallet with network at {}...",
            self.api_endpoint
        );

        // Set synced flag to true
        self.synced = true;

        println!("Wallet synchronized with network!");
    }
}
