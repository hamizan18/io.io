use std::io;

fn main(){

    let mut input = String::new();

    print!("Masukkan nilai a: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let a : i32 = input.trim().parse().unwrap();

    input.clear();

    print!("Masukkan nilai b: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let b : i32 = input.trim().parse().unwrap();

    let res = a + b;
    println!("Hasil dari {} + {} adalah: {}", a, b, res);

}