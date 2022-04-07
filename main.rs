fn main() {
    for number in 1..22 {
        if number % 15 == 0{
            println!("Print number is divide 3,5  ");
        } else if number % 3 == 0 {
            println!("Print number is divide 3  ");
        } else if number % 5 == 0 {
            println!("Print number is divide 5  ");
        } else {
            println!("{}" ,number);
        }
      
    }
}
