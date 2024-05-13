mod bill;
mod commands;
mod manager;

fn main() {
    let mut manager = crate::manager::BillManager::new();
    crate::commands::init_app(&mut manager);
}
