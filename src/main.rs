use std::io;

fn main() {
    // 1. Запрашиваем число у пользователя
    println!("Enter a number: ");

    // 2. Создаем строковую переменную для хранения ввода пользователя
    let mut input = String::new();

    // 3. Читаем строку из стандартного ввода
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // 4. Преобразуем строку в число (i32)
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return; // Завершаем программу, если ввод недействителен
        }
    };

    // 5. Выводим заголовок таблицы умножения
    println!("Table multiplication for {}", number);

    // 6. Генерируем и выводим таблицу умножения от 1 до 10
    for i in 1..=10 {
        // 7. Вычисляем произведение
        let product = number * i;

        // 8. Форматируем и выводим строку таблицы умножения
        println!("{} x {} = {}", number, i, product);
    }
}