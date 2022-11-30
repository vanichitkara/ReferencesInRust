//use -> equivalent of import used to import libraries

use std::io; //gives access to data structures

use std::env; //used to build CLI applications

fn main() {
    let mut v = Vec::new();
    v.push(8);
    v.push(3);
    v.pop();
    
    let vec = vec![1,2,3,4,5,6]; //vector macro
    let vec1 = vec!["h","e","l","o"];
    
    for i in vec.iter(){ //can use v.iter_mut() and vec.into_iter() too
        println!("{:?}",i)
    }
    //a for loop by deafult takes the ownership of the variable
    for i in &vec{ //to avoid the error of move, use &vec instead of vec
        println!("{:?}",i+2)
    }

    //Taking user input
    let mut buffer = String::new();
    let mut buffer1 = String::new();
    println!("Enter your first name");
    io::stdin().read_line(&mut buffer).expect("cannot read input"); 
    println!("Enter your last name");
    io::stdin().read_line(&mut buffer1).expect("cannot read input"); 
    println!("{:?}, {:?}", buffer.trim(), buffer1.trim()); //.trim() is used to remove the \r\n from the buffer string

    let mut args:Vec<String> = env::args().collect();

    for i in args {
        println!("{}", i); //we can add if else conditions and functions here to make a CLI program do more stuff
    }

    //This program asks for personal information if string "Info" is encountered in CLI
    let mut args:Vec<String> = env::args().collect();

    for i in args{
        if i == String::from("Info"){
            let mut firstName = String::new();
            let mut lastName = String::new();
            let mut address = String::new();
            println!("Enter first name");
            io::stdin().read_line(&mut firstName).expect("cannot read input"); 
            println!("Enter last name");
            io::stdin().read_line(&mut lastName).expect("cannot read input"); 
            println!("Enter Address");
            io::stdin().read_line(&mut address).expect("cannot read input"); 
            println!("{:?}, {:?}, {:?}", firstName.trim(), lastName.trim(), address.trim());
        }
    }
}
