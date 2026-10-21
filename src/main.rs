use rust_workspace::status::StatusFlags;

fn main() {
    let mut status = StatusFlags::empty();

    status.insert(StatusFlags::TX_READY);
    status.insert(StatusFlags::RX_READY);

    println!("Status register: {:010b}", status.bits());

    if status.contains(StatusFlags::RX_READY) {
        println!("Receiever is ready");
    }

    status.remove(StatusFlags::RX_READY);

    println!("Update status: {:#010b}", status.bits());
}
