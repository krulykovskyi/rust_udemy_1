const STARTING_MISSILES: i32 = 7;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let some_var: i32 = 15;

    println!("Firing {} missiles of {} ", ready, missiles);

    println!("{} missiles left", missiles - ready);

    READY_AMOUNT = 1;
}
