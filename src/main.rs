fn main(){

    let mut first = String::new();
    println!("Masukkan nilai a: ");
    let a = std::io::stdin().read_line(&mut first).unwrap();
    println!("Hii, {}", first);
    println!("Nilai a adalah: {}", a);

}
// fn main(){
// let mut count = 1;

//     let result = loop {
//     println!("Hello!");

//     if count < 3 {
//         break count; // Stop the loop and return the number 3
//     }

//     count += 1;
//     };

// println!("The loop stopped at: {}", result);
// }