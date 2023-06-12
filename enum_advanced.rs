enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum UsState {
    Alabama,
    Alaska,
    // ... additional states
}

fn main() {
    let my_coin = Coin::Quarter(UsState::Alabama);
    let coin_value = value_in_cents(&my_coin);
    println!("The value of the coin is {} cents.", coin_value);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

/*
In this example, we define an enum called Coin that represents different types of coins. The Coin enum has four variants: Penny, Nickel, Dime, and Quarter. The Quarter variant is associated with another enum called UsState, representing the state associated with the quarter.

In the main function, we create an instance of the Coin enum called my_coin, specifically a Quarter variant with the associated UsState::Alabama value. We then call the value_in_cents function, passing a reference to my_coin.

The value_in_cents function takes a reference to a Coin enum and matches on its variants using a match expression. Depending on the variant, the function returns a corresponding value in cents. For the Quarter variant, it also prints the associated state using a println! macro.
*/
