use std::thread;
use std::sync::{Arc,Mutex};

struct Account(i32);

impl Account {
    fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }
    
    fn withdraw(&mut self, amount: i32) {
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            panic!("Error: Insufficient funds.")
        }
    }
    
    fn balance(&self) -> i32 {
        self.0
    }
}


fn main() {
    let account: Account = Account(0);
    let mutex = Arc::new(Mutex::new(account));
    let mutex1 = Arc::clone(&mutex);
    let mutex2 = Arc::clone(&mutex);
    let mutex3 = Arc::clone(&mutex);
    let mutex4 = Arc::clone(&mutex);
    let customer1_handle = thread::spawn(move || {
        let mut account = mutex1.lock().unwrap();
        account.deposit(40);
    });
    
    let customer2_handle = thread::spawn(move || {
        let mut account = mutex2.lock().unwrap();
        account.withdraw(30);
    });
    
    let customer3_handle = thread::spawn(move || {
        let mut account = mutex3.lock().unwrap();
        account.deposit(60);
    });
    
    let customer4_handle = thread::spawn(move || {
        let mut account = mutex4.lock().unwrap();
        account.withdraw(70);
    });
    
    let handles = vec![
    customer1_handle,
    customer2_handle,
    customer3_handle,
    customer4_handle,
    ];
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let savings = mutex.lock().unwrap().balance();
    
    println!("Balance: {:?}", savings);
}