extern crate dbus;
extern crate pinyin;

mod dbus_service;
mod search_context;
mod lcs;

fn main() {
    dbus_service::start_service();
}
