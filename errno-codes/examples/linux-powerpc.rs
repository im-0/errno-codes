extern crate errno_codes;

use errno_codes::unix::linux::powerpc as linux_ppc;

fn main() {
    println!(
        "Errno with number 58 from Linux on PowerPC: {}",
        linux_ppc::info_by_num(58).unwrap()
    );
    println!(
        "Errno with identifier ENOSYS from Linux on PowerPC: {}",
        linux_ppc::info_by_id("ENOSYS").unwrap()
    );
    println!(
        "Errno message with number 32 from Linux on PowerPC: {}",
        linux_ppc::strerror(32).unwrap()
    );
}
