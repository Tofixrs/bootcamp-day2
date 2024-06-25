use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
};

use candid::CandidType;

thread_local! {
    static WPISY: RefCell<Vec<Wpis>> = RefCell::default();
}

#[derive(Debug, Clone, CandidType)]
struct Wpis {
    name: String,
    content: String,
}

#[ic_cdk::query]
fn greet(name: String, num: i8) -> String {
    format!("Hello, {name} {num}!")
}

#[ic_cdk::update]
fn dodaj_wpis(name: String, content: String) {
    WPISY.with(|wpisy| wpisy.borrow_mut().push(Wpis { name, content }))
}

#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<Wpis> {
    WPISY.with(|wpisy| wpisy.borrow().clone())
}
