// enum for payments
#[derive(Debug, PartialEq)]
enum Payment {
    CustomerId(i32),
    Amount(i32),
    AccountName(Option<String>),

    // create method to create payment
    fn create_payment(customer_id: i32, amount: i32, account_name: Option<String>) -> Payment {
        Payment {
            customer_id,
            amount,
            account_name,
        }
    }
}