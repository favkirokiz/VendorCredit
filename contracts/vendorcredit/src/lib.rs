#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String,
};

#[contracttype]
pub enum DataKey {
    Admin,
    Customer(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct Customer {
    pub name: String,
    pub phone: String,
    pub credit_limit: i128,
    pub outstanding_balance: i128,
}

#[contract]
pub struct VendorCredit;

#[contractimpl]
impl VendorCredit {
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    fn only_admin(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if &admin != caller {
            panic!("unauthorized");
        }
    }

    pub fn register_customer(
        env: Env,
        caller: Address,
        user: Address,
        name: String,
        phone: String,
        credit_limit: i128,
    ) {
        caller.require_auth();
        Self::only_admin(&env, &caller);

        let customer = Customer {
            name,
            phone,
            credit_limit,
            outstanding_balance: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Customer(user), &customer);
    }

    pub fn record_credit(env: Env, caller: Address, user: Address, amount: i128) {
        caller.require_auth();
        Self::only_admin(&env, &caller);

        if amount <= 0 {
            panic!("invalid credit");
        }

        let mut customer: Customer = env
            .storage()
            .persistent()
            .get(&DataKey::Customer(user.clone()))
            .unwrap();

        customer.outstanding_balance += amount;

        if customer.outstanding_balance > customer.credit_limit {
            panic!("credit limit exceeded");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Customer(user), &customer);
    }

    pub fn record_payment(env: Env, caller: Address, user: Address, amount: i128) {
        caller.require_auth();
        Self::only_admin(&env, &caller);

        let mut customer: Customer = env
            .storage()
            .persistent()
            .get(&DataKey::Customer(user.clone()))
            .unwrap();

        if amount <= 0 {
            panic!("invalid payment");
        }

        if amount > customer.outstanding_balance {
            panic!("overpayment");
        }

        customer.outstanding_balance -= amount;

        env.storage()
            .persistent()
            .set(&DataKey::Customer(user), &customer);
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        let customer: Customer = env
            .storage()
            .persistent()
            .get(&DataKey::Customer(user))
            .unwrap();

        customer.outstanding_balance
    }

    pub fn get_customer(env: Env, user: Address) -> Customer {
        env.storage()
            .persistent()
            .get(&DataKey::Customer(user))
            .unwrap()
    }
}