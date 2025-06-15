pub fn breaking_unsafe() {
    let mut x = 10;
    let ptr = &mut x as *mut i32;

    unsafe {
        let ur1: &mut i32 = &mut *ptr;
        let ur2: &mut i32 = &mut *ptr; // UB if r1 is still used

        *ur1 += 1; // ❌ Undefined behavior if r2 is considered active
        *ur2 += 1;
        println!("ur1 = {}", ur1);
        println!("ur2 = {}", ur2);
    }

    println!("x = {}", x);
}
