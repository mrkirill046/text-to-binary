# Текст в бинарный код

Этот проект конвертирует введённый текст в бинарный код и сохраняет результат в буфер обмена.

## Как использовать

1. Клонируйте репозиторий:
    ```bash
    git clone https://github.com/mrkirill046/text-to-binary.git
    cd text-to-binary
    ```

2. Соберите проект с помощью Cargo:
    ```bash
    cargo build --release
    ```

3. Запустите программу:
    ```bash
    cargo run
    ```

   Программа попросит вас ввести текст, который будет преобразован в бинарный код и сохранён в буфер обмена.

4. Бинарный код будет отображён в консоли и сохранён в буфер обмена.

## Требования

- Rust (https://www.rust-lang.org/)
- `clipboard` для работы с буфером обмена.
