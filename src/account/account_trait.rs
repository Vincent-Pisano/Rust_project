pub trait AccountTrait{
    fn new(username: String, password: String) -> Self;

    // Login function
    fn login(username: String, password: String) -> bool;
}

