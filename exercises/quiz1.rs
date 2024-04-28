// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)

// J'ai vu que dans les tests, dans price 1 et price 2, le prix est 70 et 80, donc doublé, donc j'ai mis que c'est le prix de base * 2
// Pour price 3, le prix est 41, donc j'ai mis que c'est le prix de base
// Pour price 4, le prix est 65, donc j'ai mis que c'est le prix de base
// J'ai mis les conditions dans la fonction pour que si la quantité est supérieure à 40, le prix soit la quantité, sinon le prix est la quantité * 2

fn calculate_price_of_apples (quantity: i32) -> i32 {
    if quantity > 40 {
        quantity
    } else {
        quantity * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
