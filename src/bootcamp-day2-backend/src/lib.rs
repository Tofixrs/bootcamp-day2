mod random;

use candid::CandidType;
use std::cell::RefCell;
use uuid::Uuid;

thread_local! {
    static WPISY: RefCell<Vec<Wpis>> = RefCell::default();
}

#[derive(Debug, Clone, CandidType)]
struct Wpis {
    name: String,
    content: String,
    id: String,
}

#[ic_cdk::update]
fn dodaj_wpis(name: String, content: String) {
    WPISY.with(|wpisy| {
        wpisy.borrow_mut().push(Wpis {
            name,
            content,
            id: Uuid::new_v4().hyphenated().to_string(),
        })
    })
}

#[ic_cdk::update]
fn usun_wpis(id: String) {
    WPISY.with(|wpisy| {
        let wpis = wpisy
            .borrow()
            .iter()
            .position(|wpis| wpis.id == id)
            .unwrap();

        wpisy.borrow_mut().remove(wpis);
    });
}

#[ic_cdk::update]
fn edit_wpis(id: String, content: String) {
    WPISY.with(|wpisy| {
        let mut wpisy = wpisy.borrow_mut();
        let wpis = wpisy.iter_mut().find(|wpis| wpis.id == id).unwrap();
        wpis.content = content;
    });
}

#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<Wpis> {
    WPISY.with(|wpisy| wpisy.borrow().clone())
}

#[ic_cdk::init]
fn init() {
    tracing_subscriber::fmt().init();
}
