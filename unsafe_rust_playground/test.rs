use std::ptr;

fn main() {
    let int_value: u32 = 10;
    let int_address = &int_value as *const u32;
    let int_address_string = format!("{:p}", int_address); // this produces something like '0x7f06092ac6d0'
    println!("{}", int_address_string);
    println!("int_value address: {:p}", int_address);
    println!("int_value value: {:?}", unsafe { *int_address });
    let get_int_address_value = unsafe { ptr::read(int_address) };
    println!("{}", get_int_address_value);
}
