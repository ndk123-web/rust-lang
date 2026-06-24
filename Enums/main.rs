/**
 * Enums for (Either x or y) not both thats the entire idea
 * Example: let a = 10 now here either its 10 or None this is both possibility
 */

enum UpiMethods {
    Gpay,
    PayPal,
}

enum Payments {
    Cash,
    Upi(UpiMethods),
    Card(String),
}

fn matchBro(role: Payments) {
    // so this is for variant checking only
    match role {
        Payments::Cash => {
            println!("Cash");
        }

        Payments::Upi(UpiMethods::Gpay) => {
            println!("Upi Gpay");
        }

        Payments::Upi(UpiMethods::PayPal) => {
            println!("Upi PayPal");
        }

        Payments::Card(id) => {
            println!("Cash - {}", id);
        }
    }
}

fn main() {
    let obj1 = Payments::Cash;
    let obj2 = Payments::Upi(UpiMethods::Gpay);
    let obj3 = Payments::Upi(UpiMethods::PayPal);

    matchBro(obj1);
    matchBro(obj2);
    matchBro(obj3);
}
