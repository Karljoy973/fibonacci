
/// finbonacci(x:i32) -> i64
/// Evalue la valeur du n-ième terme de la suite de fibonacci de façon itérative     
pub fn fibonacci(x: i32)-> i64 {
        if x<0 {panic!("Invalid Input");}
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