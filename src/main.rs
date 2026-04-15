use qrcode::QrCode;
use std::io;

fn main() {
    println!("Введите текст или URL:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let code = QrCode::new(input.as_bytes()).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    println!("\nQR-код:\n{}", string);
}