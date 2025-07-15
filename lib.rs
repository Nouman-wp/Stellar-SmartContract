#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol};

pub struct CounterContract;

#[contractimpl]
impl CounterContract {

    pub fn initialize(env: Env) {
        env.storage().set(Symbol::short("count"), &0i32);
    }


    pub fn increment(env: Env) {
        let key = Symbol::short("count");
        let mut count: i32 = env.storage().get_unchecked(key).unwrap();
        count += 1;
        env.storage().set(key, &count);
    }


    pub fn get(env: Env) -> i32 {
        let key = Symbol::short("count");
        env.storage().get_unchecked(key).unwrap()
    }
}

