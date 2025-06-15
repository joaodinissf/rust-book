pub fn breaking_unsafe_v2() {
    let mut val1 = 1;
    let mut val2 = 2;

    let mut val1_ref = &val1;
    let mut val2_ref = &val2;

    let val1_ptr = &raw const val1;
    let val2_ptr = &raw const val2;

    unsafe {
        val2_ref = &mut *val1_ptr;
    }

    println!("val1: {}, val2: {}", val1, val2);
    println!("val1_ref: {:p}, val2_ref: {:p}", val1_ref, val2_ref);
    println!("val1_ptr: {:p}, val2_ptr: {:p}", val1_ptr, val2_ptr);
}
