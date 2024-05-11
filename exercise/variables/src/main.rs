const STARTING_MISSILES: i32 = 7;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let some_var: i32 = 15;

    println!("Firing {} missiles of {} ", ready, missiles);

    println!("{} missiles left", missiles - ready);

    //READY_AMOUNT = 1;
    //помилка компіляції: спроба перезаписання константи

    {
        println!("Upper scope variable value: {}", some_var);
    }
}

// Висновки:
// Всі змінні доступні лише в своєму блоці видимості, та у вкладених блоках,
// але змінні, які було створено через к.с. "const" доступні глобально
