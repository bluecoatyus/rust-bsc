struct Customer {
    name: String,
    surname: String,
    balance: i32,
}

struct Product {
    name: String,
    price: u32,
    stock_quantity: u32,
}

// Market functions

impl Customer {
    // Function for purchasing a product
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        // If the product is available in sufficient quantity in stock and the customer has enough balance to buy it, 
        // purchase the product and update the customer's balance. Return true if the purchase is successful, false otherwise.
        if product.stock_quantity >= quantity && self.balance >= quantity as i32 * product.price as i32 {
            product.stock_quantity -= quantity;
            self.balance -= quantity as i32 * product.price as i32;
            return true;
        }
        false
    }
}

// Main program

fn main() {
    let mut customer1 = Customer {
        name: String::from("Yunus Emre"),
        surname: String::from("Yalin"),
        balance: 150,
    };

    let mut customer2 = Customer {
        name: String::from("Metin"),
        surname: String::from("Yalin"),
        balance: 20,
    };

    let mut product = Product {
        name: String::from("Apple"),
        price: 5,
        stock_quantity: 20,
    };

    // Customers' product purchase operations
    println!("Customer 1 is buying a product...");

    if customer1.buy_product(&mut product, 13) {
        println!("Customer 1 successfully purchased the product.");
    } else {
        println!("Customer 1 couldn't purchase the product.");
    }

    println!("Customer 2 is buying a product...");

    if customer2.buy_product(&mut product, 5) {
        println!("Customer 2 successfully purchased the product.");
    } else {
        println!("Customer 2 couldn't purchase the product.");
    }
}
