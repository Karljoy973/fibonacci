use std::io; 

fn main() {

/// finbonacci(x:i32) -> i64
/// Evalue la valeur du n-ième terme de la suite de fibonacci de façon itérative     
    fn fibonacci(x: i32)-> i64 {
        if x<0 {return x.into(); }
        let mut i: i64= 0;
        if x == 0 { return i;}
        let mut j: i64 = 1;
        if x == 1 { return j;}
        let mut l = 0;
        while  l<x{
            if l%2 == 0  {i = i+j;};
            if l%2 == 1 { j = i+j};
            l = l+1;
        }
        if i>j {return i;};
        return j;
    }
    println!("Enter the n-th fibonacci element you seek for ?");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Input failed \n");
    let initial_value = input.trim();
    let input:i64 = match input.trim().parse() {
        Ok(num) => fibonacci(num),
        Err(_) => fibonacci(-1),
    };
   if input >= 0 {
    println!("Le {initial_value}-ème terme de la suite de fibonacci vaut {input}")
   }
   else{
    println!("Ce terme n'existe pas, veuillez relancer le programme avec un nombre supérieur ou égal à zéro.")
   }
}
