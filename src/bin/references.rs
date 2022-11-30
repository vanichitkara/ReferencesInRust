fn main() {
    //pointers in other languages -> can point to invalid value
    // reference in Rust -> cannot point to invalid value

    let str = String::from("value");
    calculate_length(&str); //ownership passes to the function if & is not used
    println!("{}",str); //ownership is not returned back hence str is invalid when reference is not used

    //to avoid passing of ownership, clone or reference can be used
    //cloning increases memory as it creates a new variable in heap, hence reference is more suitable

    //mutate_data(&str); // reference to str is immutable by default and hence cannot be mutated

    //Mutable reference
    let mut st = String::from("hello");
    mutate_data(&mut st); //now the string is mutable

    let r1 = &mut st;
    let r2 = &mut st; //writers -  they can modify the data
    let r3 = &st; //readers- they can only take the reference but cannot modify the data
    //cannot borrow a string mutably and immutably at the same time

    println!("{:?}, {:?}", r1, r2); //you cannot take 2 or more than 2 references of a value 
    //They can be mutated differently at the same time causing an error

    println!("{:?}, {:?}", r3, r1);

    //Rust enforces a single writer or multiple readers rule. Either you can read and write the value,
    //or it can be shared by any number of readers, but never both at the same time

    let res = return_ref(); //dangling reference

    let a:&String;
    {
        let s = String::from("value");
        a = &s;
    }//s will be deleted after its scope ends but a is still referencing to s
    
    //Any borrow must last for a scope no greater than that of the owner
    //We can have one or more immutable reference to a data or have exactly one mutable reference to the data
}

//&String is used instead of String as the reference of str is being passed
fn calculate_length (s:&String) -> usize {
    s.len()
}

fn mutate_data(s:&mut String){
    let ms =  s.push_str("abc");
    println!("{:?}",ms);
}

fn return_ref() -> &String {
    let s = String::from("value");
    &s;
} // s will be deleted when this code block is executed, but its reference is passed to ref at line 31