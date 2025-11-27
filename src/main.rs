fn main() {
    println!("Hello, world!");
}

#[test]
fn hello() {
    println!("Aku aslinya dua orang");
}

#[test]
fn variable() {
    let name = "Sansgrids";
    let umur = 21;
    let umur_asli = umur + 1;
    println!("My name is {} and i'm {} years old", name, umur_asli);
}

#[test]
fn mutable() {
    let mut name = "Sansgrids";
    println!("Hello, {}", name);

    name = "Ihsan Gadun";
    println!("Hello, {}", name);
    
    name = "Jekol";
    println!("Hello, {}", name);
}