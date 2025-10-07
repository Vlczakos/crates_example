/*
Vystup:

Program 2
Toto je funkce v crate 1
Nahodne cislo: <cislo>
*/

fn main() {
    println!("Program 2");
    crate1::test_function();

    let i = rand::random::<u32>();
    println!("Nahodne cislo: {}", i);
}
