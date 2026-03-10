// References and Borrowing 
// Safety and Performance
// Borrowing and references are powerful concepts.

// In the last lesson (ownership), we understood that how ownership is very unique to Rust programming language and how ownership is very important for memory management.

// In Rust, managing memory is crucial, for ensuring both safety and performance. But what does the word safety mean here?
// Alot of programmer youtubers talk alot about safety in Rust, but no one seems to explain in details what safety means, why safety is important in programming language and what is safety to begin with?

// As a matter of fact, safety referes to the prevention of certain types of bugs and errors that commonly occur in other languages like in C and C++. 
// Things like null pointer dereferencing, dangling pointers, buffer overflows, and data races. (There would be videos to explain each of those errors).

// Understanding References.
// References: Enable you to borrow values without taking ownership
// Immutable referencce.
// Mutable Reference.
// Create Reference by adding "&"
// ------------------------
// Actually borrowing and references are the same thing, well basically, you create references by borrowing from the original owner of that value.
// As we have explained in the last lesson, we have only ONE OWNER, which is the variable that has only one value. So you cannot have multiple owners for the same value.

// So let's say references in Rust enables you to borrow values without taking the ownership.
// This is very important for efficient memory management, actually rust doesn't allow you to have multiple owners for the same value. That's why we have to create references by borrowing from the owner.

// It's very important to know that references can be both immutable and mutable.
// so immutable references allow you borrowing without modification, and of course, mutable references allows you borrowing with the ability to modify the data.

// And to creatte a reference, simply you will add the ampersand before the variable you're referring to.

// fn main() {
//     let _x: i32 = 5;
//     let _r = &_x;
//     println!("Value of x: {}", _x);
//     println!("Value of r: {}", _r);
// }
// As you can see value of x is equal to 5 and also the value of r is equal to 5
// r being the reference to x and it has the same value as x but it's not the owner of that value 5, but rather it's referring to that original owner of that value 5.

// II - Mutable Reference
// mutable reference simply mean that you can modify, you can increment, you can decrement, you can modify that value.
// fn main() {
//     let mut _x: i32 = 5;
//     let _r: &mut i32 = &mut _x;
    
//     *_r += 1;
//     *_r -= 3;
    
//     println!("Value of x: {}", _x);
//     // println!("Value of x: {}", _r);
// }
// So we need to add the mut keyword to the reference, but not only that, but also to the owner.

// you can see that cannot borrow '_x' as immutable because it's also borrowed as mutable. Immutable borrow occurs here

// This is on very important rule, you can only have one mutable reference, or many immutable references.

// You can either have one mutable reference to a value or any number of immutable references.

// -------
// STRUCT:
// A data structure that allows you to group multiple fields together under one name.
// Demonstration on one mutable reference or many immutable references.
fn main() {
    let mut account: BankAccount = BankAccount {
         owner: "Frank".to_string(),
         balance: 21315.0,
    }; 
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(115.0);

    account.check_balance();
}

// Suppose we have a struct representing a bank account, 
struct BankAccount {
    owner: String, 
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}

// With this withdraw function, I want to ensure that we cannot simultaneously have mutable access to the account to update the balance and immutable access for reading the owner's name, for example.

// so for the check_balance fn, I want to ensure that while we are checking the balance, which has immutable access, no other part of our code is modyfying the balance, which has immutable access. The way to implement that is, we're going to simply...

// in the main function we create a mutable variable


// Again in this lesson, you can only have one mutable borrow, but many immutable borrow. But you cannot have both at the same time.
// The reason why the code compile successfully because each borrow the 'check_balance' and the 'withdraw' here in both functions, each is in its own scope. So they do not overlap, and therefore there is no simultaneous mutable and immutable borrowing of account.
