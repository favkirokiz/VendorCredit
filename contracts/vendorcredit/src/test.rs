#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, Address, String};

fn env() -> Env {
    Env::default()
}

#[test]
fn happy_path() {
    let env = env();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    VendorCredit::init(env.clone(), admin.clone());

    VendorCredit::register_customer(
        env.clone(),
        admin.clone(),
        user.clone(),
        String::from_str(&env, "Juan"),
        String::from_str(&env, "0917"),
        1000,
    );

    VendorCredit::record_credit(env.clone(), admin.clone(), user.clone(), 500);
    VendorCredit::record_payment(env.clone(), admin.clone(), user.clone(), 200);

    let bal = VendorCredit::get_balance(env.clone(), user.clone());
    assert_eq!(bal, 300);
}

#[test]
fn unauthorized_access() {
    let env = env();

    let admin = Address::generate(&env);
    let hacker = Address::generate(&env);
    let user = Address::generate(&env);

    VendorCredit::init(env.clone(), admin.clone());

    let res = std::panic::catch_unwind(|| {
        VendorCredit::record_credit(env.clone(), hacker.clone(), user.clone(), 100);
    });

    assert!(res.is_err());
}

#[test]
fn invalid_credit() {
    let env = env();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    VendorCredit::init(env.clone(), admin.clone());

    let res = std::panic::catch_unwind(|| {
        VendorCredit::record_credit(env.clone(), admin.clone(), user.clone(), -10);
    });

    assert!(res.is_err());
}

#[test]
fn state_verification() {
    let env = env();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    VendorCredit::init(env.clone(), admin.clone());

    VendorCredit::register_customer(
        env.clone(),
        admin.clone(),
        user.clone(),
        String::from_str(&env, "Juan"),
        String::from_str(&env, "0917"),
        1000,
    );

    VendorCredit::record_credit(env.clone(), admin.clone(), user.clone(), 700);

    let customer = VendorCredit::get_customer(env.clone(), user.clone());
    assert_eq!(customer.outstanding_balance, 700);
}

#[test]
fn overpayment_rejected() {
    let env = env();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    VendorCredit::init(env.clone(), admin.clone());

    VendorCredit::record_credit(env.clone(), admin.clone(), user.clone(), 300);

    let res = std::panic::catch_unwind(|| {
        VendorCredit::record_payment(env.clone(), admin.clone(), user.clone(), 500);
    });

    assert!(res.is_err());
}