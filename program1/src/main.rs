/*
Vystup:

Program 1
Toto je funkce v crate 1
Toto je funkce v crate 2
*/

fn main() {
    println!("Program 1");
    crate1::test_function();
    crate2::test_function();
}
