use std::io;
use std::process::exit;

fn pythagorean_triples(s: u64, t: u64) -> (u64, u64, u64) {

        let a: u64 = s*t;
        let b: u64 = (s.pow(2) - t.pow(2))/2;
        let c: u64 = (s.pow(2) +t.pow(2))/2;
        return (a,b,c);
}

fn is_pythagorean_triple (x: (u64,u64,u64)) -> bool {
         let m: u64 = (x.0).pow(2) + (x.1).pow(2);
         let n: u64 = (x.2).pow(2);
         if m == n {
            true
         }
         else {
            false
         }
}

fn main() {

    let mut s = String::new();
    println!("Please enter the first number:");
    io::stdin().read_line(&mut s).expect("Number not entered");
    let s: u64 = s.trim().parse().expect("Please type a number");

    let mut t = String::new();
    println!("Please enter the second number:");
    io::stdin().read_line(&mut t).expect("Number not entered");
    let t: u64 = t.trim().parse().expect("Please type a number");
    
    if s > 0 && t > 0 && s > t && s%2 !=0 && t%2 != 0{

           let x = pythagorean_triples(s,t);
           println!("The generated Pythagorean triples are {:?}",x);
           println!("\n");
           println!("Let's check if indeed the numbers {:?} are Pythagorean triples", x);
           println!("\n");
           println!("Checking........");
           println!("\n");
           if is_pythagorean_triple(x) == true {
            println!("Indeed the numbers {:?} are Pythagorean triples", x);
            println!("\n");

           }
           else {
            println!("Hmmm the program seems to be buggy");
            println!("\n");
           }
        }

    else {
        println!(" You have messed up something. One of the numbers entered is an even number or zero or ordered incorrectly");
        exit(0);
    }
}

