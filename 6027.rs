use std::collections::HashMap;

fn main() {
    for r8 in 1..32768 {
        if init(r8) == 6 {
            println!("In {}, Out {}", r8, init(r8));
        }
    }
}

fn init(r8: u32) -> u32 {
    let mut cache = HashMap::new();
    call6027(4, 1, r8, &mut cache)
}

fn call6027(r1: u32, r2: u32, r8: u32, cache: &mut HashMap<(u32, u32), u32>) -> u32 {
    if let Some(val) = cache.get(&(r1, r2)) {
        return *val;
    }
    let ret = if r1 != 0 {
        if r2 != 0 {
            call6027(r1 - 1, call6027(r1, r2 - 1, r8, cache), r8, cache)
        } else {
            call6027(r1 - 1, r8, r8, cache)
        }
    } else {
        (r2 + 1) % 32768
    };

    cache.insert((r1, r2), ret);
    ret
}
