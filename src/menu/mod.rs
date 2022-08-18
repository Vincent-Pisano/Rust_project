use crate::account::client::*;
use crate::account::credentials::*;

pub fn show_menu(mut client: Client) {
    crate::menu::menu::show_menu(client)
}

pub fn enter_password(username: &String) -> Option<Client> {
    crate::menu::menu::enter_password(username)
}

pub fn sign_up() -> Client {
    crate::menu::menu::sign_up()
}

mod menu {
    use std::io;
    use std::process;

    use crate::account::client::*;
    use crate::account::credentials::Credentials;
    use crate::database::*;

    enum Operations {
        Add,
        Subtract,
    }

    // SHOW MENU

    pub fn show_menu(mut client: Client) {
        println!("Hello {} !", client.to_string());
        loop {
            println!(
                "\nWhat Would you like to do today?
        1. Check your balance
        2. Make a deposit
        3. Make a withdrawal
        Q. Swallow your tears and leave you poor fuck",
            );
            client = enter_menu_option(client);
        }
    }

    fn enter_menu_option(mut client: Client) -> Client {
        match ask_input("line").to_ascii_lowercase().as_str().trim() {
            "1" => {
                println!("You currently have {}$", client.balance());
            }
            "2" => {
                println!("Please enter the amount you wish to deposit");
                client = enter_amount_to_modify(client, Operations::Add);
            }
            "3" => {
                println!("Please enter the amount you wish to withdraw");
                client = enter_amount_to_modify(client, Operations::Subtract);
            }
            "q" => {
                println!("Bye");
                process::exit(0);
            }
            _ => {
                println!("That's not a valid choice...");
            }
        };
        client
    }

    fn enter_amount_to_modify(mut client: Client, operation: Operations) -> Client {
        let amount: f32 = ask_input("amount").trim().parse::<f32>().unwrap_or(-1f32);
        match amount > 0f32 {
            true => match operation {
                Operations::Add => add_amount_to_balance(client, amount),
                Operations::Subtract => remove_amount_from_balance(client, amount),
            },
            false => {
                println!("The entered value was invalid... Please enter a number above 0");
                client //return unchanged client
            }
        }
    }

    fn add_amount_to_balance(mut client: Client, amount: f32) -> Client {
        modify_balance(client.credentials(), *client.balance() + amount);
        client.set_balance(*client.balance() + amount);
        client
    }

    fn remove_amount_from_balance(mut client: Client, amount: f32) -> Client {
        if *client.balance() >= amount {
            modify_balance(client.credentials(), *client.balance() - amount);
            client.set_balance(*client.balance() - amount);
        } else {
            println!(
                "It seems that you do not have the required money...\nYour current balance is {}$",
                *client.balance()
            )
        }
        client
    }

    // SIGN UP

    pub fn sign_up() -> Client {
        let username = ask_input("username");
        let password = ask_input("password");

        let credentials = &Credentials::new(username.trim().to_string(), password.trim().to_string());

        match verify_credentials(credentials) {
            None => {
                let mut balance = ask_input("balance");

                register_new_client(
                    credentials,
                    balance.trim().to_string(),
                )
            }
            Some(optional_client) => {
                println!("Oops, it seems that username is already taken...");
                sign_up()
            }
        }
    }

    // LOGIN

    pub fn enter_password(username: &String) -> Option<Client> {
        let mut password = ask_input("password");

        match verify_credentials(&Credentials::new(username.trim().to_string(), password.trim().to_string())) {
            None => {
                println!("No account was found with those credentials...\nPlease Sign up");
                None
            }
            Some(optional_client) => Some(optional_client),
        }
    }

    // ASK INPUT

    fn ask_input(input_name: &str) -> String {
        print!("Enter your {}: \n", input_name);
        let msg = format!("Failed to read the {}", input_name);
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect(&msg);
        input
    }
}
