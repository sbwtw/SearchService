
extern crate dbus;
extern crate pinyin;

mod dbus_service;
mod search_context;

fn main() {
    dbus_service::start_service();
}
