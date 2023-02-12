fn main() {
    println!("Out: {}", init(32767));
}

fn init(r8: u32) -> u32 {
    call6027(4, 1, r8)
}

fn call6027(r1: u32, r2: u32, r8: u32) -> u32 {
    if r1 != 0 {
        if r2 != 0 {
            call6027(r1 - 1, call6027(r1, r2 - 1, r8), r8)
        } else {
            call6027(r1 - 1, r8, r8)
        }
    } else {
        (r2 + 1) % 32768
    }
}