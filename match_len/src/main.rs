fn main() {
    println!("Hello, world!");
    value_in_cents(Coin::Quarter(UsState::Alabama));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        //Coin::Dime => 10,
        Coin::Quarter(sate) => {
            println!("UsState is {sate:?}");
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
