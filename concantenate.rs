fn main(){

    let first_sample = "Hello"
    let second_sample = "world!"
    
    println!(concantenate_string(&first_sample, &second_sample))

}

fn concantenate_string(first_string: &String, second_string: &String )-> String{
    let mut result = first_string.clone()
    result.push_str(second_string.clone())
    result
}