use candid::{CandidType, Nat};
use std::cell::RefCell;

thread_local! {
    static WPISY: RefCell<Vec<Wpis>> = RefCell::default();
}

#[derive(Debug, Clone, CandidType)]
struct Wpis {
    name: String,
    content: String,
}

#[ic_cdk::update]
fn dodaj_wpis(name: String, content: String) {
    WPISY.with(|wpisy| wpisy.borrow_mut().push(Wpis { name, content }))
}

#[ic_cdk::update]
fn usun_wpis(id: usize) {
    WPISY.with(|wpisy| {
        let length = wpisy.borrow().len();
        if id > length {
            return;
        };

        wpisy.borrow_mut().remove(id);
    });
}

#[ic_cdk::update]
fn edit_wpis(id: usize, wpis: String) {
    WPISY.with(|wpisy| {
        let mut wpisy = wpisy.borrow_mut();
        let Some(wpis_data) = wpisy.get_mut(id) else {
            return;
        };

        wpis_data.content = wpis;
    })
}

#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<Wpis> {
    WPISY.with(|wpisy| wpisy.borrow().clone())
}
