use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, Write};

fn main() {
    print!("Введите текст: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении ввода");

    let input = input.trim();

    let binary = input
        .chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .join(" ");

    let mut ctx: ClipboardContext =
        ClipboardProvider::new().expect("Не удалось создать контекст буфера обмена");
    ctx.set_contents(binary.clone())
        .expect("Не удалось сохранить данные в буфер обмена");

    println!("Бинарный код сохранен в буфер обмена: \n{}", binary);
}
