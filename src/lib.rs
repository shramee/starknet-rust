mod starknet_bindings;
use starknet_bindings::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logAny(s: bool);
}

#[wasm_bindgen]
pub fn greet() {
    let s = String::from("");
    let st = " yo ";
		st.trim
    st[i..j];
    let _ = s.chars().enumerate();
    if STARKNET.is_connected() {
        log("Connected!");
    } else {
        log("Not connected");
    }
}

#[wasm_bindgen(start)]
pub async fn run() {
    log("Connecting...");
    log_conn_status();
    connect().await;
    log_conn_status();
}

fn log_conn_status() {
    if STARKNET.is_connected() {
        log("Connected!");
    } else {
        log("Not connected");
    }
}

pub async fn connect() {
    STARKNET.enable().await;
}



// hashmaps2.rs

// A basket of fruits in the form of a hash map is given. The key
// represents the name of the fruit and the value represents how many
// of that particular fruit is in the basket. You have to put *MORE
// THAN 11* fruits in the basket. Three types of fruits - Apple (4),
// Mango (2) and Lychee (5) are already given in the basket. You are
// not allowed to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

		let s = String::from("");
    for fruit in fruit_kinds {
			for fruit in fruit_kinds {
				// Checks if the value is none
        if basket.get(&fruit).is_none() {
            basket.insert(fruit, 3);
        }
				// Unwraps with a default value
        if 0 == *basket.get(&fruit).unwrap_or(&0) {
            basket.insert(fruit, 3);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }
}
