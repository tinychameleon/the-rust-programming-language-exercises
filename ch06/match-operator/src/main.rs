enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn print_value(coin: Coin) {
    match coin {
        Coin::Quarter(ref state) =>
            println!("worth {} cents (from {})", value_in_cents(&coin), state),
        _ =>
            println!("worth {} cents", value_in_cents(&coin)),
    }
}

fn main() {
    print_value(Coin::Penny);
    print_value(Coin::Nickel);
    print_value(Coin::Dime);
    print_value(Coin::Quarter("California".to_string()));
}
