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

#[test]
fn static_type() {
    let mut name = "Ihsan";
    println!("Hello, {}", name);
    
    // name = 1;
    name = "Tampan";
    println!("Hello, {}", name);
}

#[test]
fn shadowing() {
    let name = "Sansgrids";
    println!("Hello, {}", name);

    let name = "Ihsan Gadun";
    println!("Hello, {}", name);

    let mut name = 1;
    println!("Hello, {}", name);

    name = 2;
    println!("Hello, {}", name);

    let name = true;
    println!("Hello, {}", name);
}
