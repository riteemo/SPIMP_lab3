use image::Luma;
use qrcode::QrCode;
use std::io;

fn main() {
    println!("Введите текст или URL:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения текста или URL");
    let input = input.trim();

    if input.is_empty() {
        eprintln!("Введите непустую строку.");
        return;
    }

    let code = QrCode::new(input.as_bytes()).expect("Ошибка создания QR-кода");
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    println!("\nQR-код:\n{}", string);

    let image = code.render::<Luma<u8>>().build();
    image.save("qr_code.png").expect("Ошибка сохранения qr_code.png");
    println!("QR-код сохранён в qr_code.png");
}