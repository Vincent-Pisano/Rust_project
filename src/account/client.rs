use super::credentials::*;

// #[derive(Default)] utilise seulement si valeur de base dans la struct | les ajout automatiquement quand on créer le struct
pub struct Client {
    credentials: Credentials,
    balance: u32,
}

impl Client {
    pub fn new(username: String, password: String, balance: u32) -> Client {
        let credentials = Credentials::new(username, password);
        Client { credentials, balance }
    }

    pub fn to_string(&self) -> String {
        self.credentials.to_string()
    }

    // Immutable access.
    fn username(&self) -> &String {
        &self.credentials.username()
    }
    fn password(&self) -> &String {
        &&self.credentials.password()
    }
    fn balance(&self) -> &u32 {
        &self.balance
    }
}