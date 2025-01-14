pub fn print_title() {
    println!("Enter the n-th fibonacci element you seek for ?");

}

pub fn print_output (initial_value: &str ,input: i64){
    if input >= 0 {
    println!("Le {initial_value}-ème terme de la suite de fibonacci vaut {input}")
   }
   else{
       println!("Ce terme n'existe pas, veuillez relancer le programme avec un nombre supérieur ou égal à zéro.")
    }
}