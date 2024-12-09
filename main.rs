use std::io;

fn main() {
    // Initial account balance
    let mut account_balance: f32 = 10000.0;

    // ATM card verification
    let atm_card: char = 'C';
    println!("Enter your card:");

    let mut atm_card_input = String::new();
    io::stdin()
        .read_line(&mut atm_card_input)
        .expect("Failed to read the line");

    let atm_card_final_input: Result<char, _> = atm_card_input.trim().parse();

    match atm_card_final_input {
        Ok(input_char) if atm_card_input.trim().len() == 1 => {
            if input_char == atm_card {
                println!("Card accepted! Welcome to RUST ATM.");
            } else {
                println!("Invalid card, please try again.");
                return;
            }
        }
        Ok(_) => {
            println!("Card input should be a single character!");
            return;
        }
        Err(_) => {
            println!("Please enter a valid character.");
            return;
        }
    }

    // Card PIN verification
    let card_pin: u32 = 1234;
    println!("Enter your card pin:");

    let mut card_input = String::new();
    io::stdin()
        .read_line(&mut card_input)
        .expect("Failed to read the line");

    let card_pin_input: Result<u32, _> = card_input.trim().parse();

    match card_pin_input {
        Ok(input_pin) if card_input.trim().len() == 4 => {
            if input_pin == card_pin {
                println!("Pin correct!");
            } else {
                println!("Incorrect pin, try again!");
                return;
            }
        }
        Ok(_) => {
            println!("Pin should be exactly 4 digits!");
            return;
        }
        Err(_) => {
            println!("Please enter a valid numeric pin.");
            return;
        }
    }

    // ATM Operations Menu
    let card_operations = "withdraw, transfer, pay bills, and check balance";
    println!("Available operations: {}", card_operations);
    println!("Enter an operation:");

    let mut operation_input = String::new();
    io::stdin()
        .read_line(&mut operation_input)
        .expect("Failed to read input");
    let operation = operation_input.trim();

    match operation {
        "withdraw" => {
            println!("You selected 'withdraw'. Please enter the amount to withdraw:");

            let mut amount_input = String::new();
            io::stdin()
                .read_line(&mut amount_input)
                .expect("Failed to read amount");
            let amount: Result<f32, _> = amount_input.trim().parse();

            match amount {
                Ok(amount) if amount > 0.0 => {
                    if amount <= account_balance {
                        account_balance -= amount;
                        println!("You have successfully withdrawn ${:.2}.", amount);
                    } else {
                        println!("Insufficient balance. Your current balance is ${:.2}.", account_balance);
                    }
                }
                Ok(_) => println!("Invalid amount. Withdrawal amount must be greater than zero."),
                Err(_) => println!("Please enter a valid number."),
            }
        }
        "transfer" => {
            println!("You selected 'transfer'. Please enter the account number:");

            let mut account_number = String::new();
            io::stdin()
                .read_line(&mut account_number)
                .expect("Failed to read account number");

            println!("Enter the amount to transfer:");

            let mut transfer_amount = String::new();
            io::stdin()
                .read_line(&mut transfer_amount)
                .expect("Failed to read transfer amount");
            let amount: Result<f32, _> = transfer_amount.trim().parse();

            match amount {
                Ok(amount) if amount > 0.0 => {
                    if amount <= account_balance {
                        account_balance -= amount;
                        println!(
                            "You have successfully transferred ${:.2} to account {}.",
                            amount, account_number.trim()
                        );
                    } else {
                        println!("Insufficient balance. Your current balance is ${:.2}.", account_balance);
                    }
                }
                Ok(_) => println!("Invalid amount. Transfer amount must be greater than zero."),
                Err(_) => println!("Please enter a valid number."),
            }
        }
        "pay bills" => {
            println!("You selected 'pay bills'. Please enter the biller name:");

            let mut biller_name = String::new();
            io::stdin()
                .read_line(&mut biller_name)
                .expect("Failed to read biller name");

            println!("Enter the amount to pay:");

            let mut bill_amount = String::new();
            io::stdin()
                .read_line(&mut bill_amount)
                .expect("Failed to read bill amount");
            let amount: Result<f32, _> = bill_amount.trim().parse();

            match amount {
                Ok(amount) if amount > 0.0 => {
                    if amount <= account_balance {
                        account_balance -= amount;
                        println!(
                            "You have successfully paid ${:.2} to {}.",
                            amount, biller_name.trim()
                        );
                    } else {
                        println!("Insufficient balance. Your current balance is ${:.2}.", account_balance);
                    }
                }
                Ok(_) => println!("Invalid amount. Payment amount must be greater than zero."),
                Err(_) => println!("Please enter a valid number."),
            }
        }
        "check balance" => {
            println!("Your current balance is ${:.2}.", account_balance);
        }
        _ => {
            println!("Invalid operation. Please select a valid operation from the list.");
        }
    }
}
