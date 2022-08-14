use super::credentials::*;

// #[derive(Default)] utilise seulement si valeur de base dans la struct | les ajout automatiquement quand on créer le struct
#[derive(Clone)]
pub struct Client {
    id: u32,
    credentials: Credentials,
    balance: f32,
}

impl Client {
    pub fn new(id: u32, username: String, password: String, balance: f32) -> Client {
        let credentials = Credentials::new(username, password);
        Client {
            id,
            credentials,
            balance,
        }
    }

    pub fn to_string(&self) -> String {
        self.credentials.username().to_string()
    }

    // Immutable access.
    pub fn id(&self) -> &u32 {
        &self.id
    }
    pub fn balance(&self) -> &f32 {
        &self.balance
    }
    // Mutable access.
    pub fn mut_balance(&mut self) -> &mut f32 {
        &mut self.balance
    }

    //setters
    pub fn set_balance(&mut self, new_balance: f32) {
        self.balance = new_balance;
    }
}
