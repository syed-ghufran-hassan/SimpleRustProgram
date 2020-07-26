use std::io;

fn main() {

let mut input = String::new();

    //println!("Enter the Number please");

    io::stdin().read_line(&mut input);

    let mut input: i8 = input.trim().parse().unwrap(); 

if input == 1
{
    println!("weird");
}    
else if input%2 == 0  && input == 2  || input == 4 
{

    println!("not weird");

}
else if input%2 != 0  && input == 3  || input == 5 
{

    println!("weird");

}


else if input%2 == 0 && input == 6 ||  input == 8 || input == 10 || input == 12 || input == 14 || input == 16 || input == 18 || input == 20 || input == 29 
{

    println!("weird");
}
else if input%2 == 0 && input == 7 &&  input == 9 && input == 11 && input == 13 && input == 15 && input == 17 && input == 19
{
println!("not weird");
}
else
{
    println!("not weird");
}
}

// Enter your code here 
 
//use std::io;

 
//fn main() {
    
  //  let mut number = String::new();
   // println!("Store Name:");
    //match io::stdin().read_line(&mut number)
    //{
      //  Ok(_) => {
        // println!("Store Name is{}",number);
        //},
        //Err(e)=> println!("Invalid Entry!: {}",e)
    
    //}
    //println!("Number of Products are:");
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).expect("Error reading input");
    //let input:i8 = input.trim().parse().expect("Error parsing number!");
    //let mut number1 = String::new();
    //println!("Product Name:");
    //match io::stdin().read_line(&mut number1)
    //{
      //  Ok(_) => {
       //  println!("Product is{}",number1);
        //},
        //Err(e)=> println!("Invalid Entry!: {}",e)
    
    //}
    //println!("The product is {} and price is Rs. 2500", number1);

//}
    //io::stdin().read_line(&mut number).expect("Error reading input");
   // let number:i8 = number.trim().parse().expect("Error parsing number!");
    

    
