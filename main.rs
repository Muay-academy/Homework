fn main() {
    for number in 1..101 {
        if number % 3 == 0 && number % 5 == 0{
            println!("3 และ 5 หารลงตัวด้วย :{}" ,number);
        } else if number % 3 != 0 && number % 5 == 0{
            println!("5 หารลงตัวด้วย :{}" ,number);
        } else if number % 3 == 0 && number % 5 != 0{
            println!("3 หารลงตัวด้วย :{}" ,number);
        } else if number % 3 != 0 && number % 5 != 0{
            
        }
      
    }
}