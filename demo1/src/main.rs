fn main() {
    println!("Hello, world!");
    for i in 1..10{
        println!("Current iterator :{}",i);
    }

    let result = evennum(22);

}

fn evennum(num : i32){
    if num %2 == 0{
      println!("{} is even" , num);
    }else{
        println!("{} is odd", num);
    }
}
