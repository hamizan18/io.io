use std::io;

fn main(){

    let mut input = String::new();

    print!("Masukkan nama anda: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let nama = input.trim().to_string();
    input.clear();

    
    print!("Masukkan umur anda sebelum memulai: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let umur: u32 = input.trim().parse().unwrap();
    
    println!("\nNama anda adalah: {}", nama.trim());
    println!("Anda berumur {}", umur);

    input.clear();
    print!("\nMasukkan angka pertama: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();

    input.clear();
    print!("Masukkan nilai b: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();

    let result = a + b;
    
    println!("Hasil dari penjumlahan {} + {} adalah {}", a, b, result);
}