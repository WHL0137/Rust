fn main() {
    let a = 10; // Tipo inferido como i32
    let b = 20u8; // Tipo especificado como u8

    let c = a + b as i32; // 'b' é convertido para i32 antes da adição

    println!("c: {}", c);
}
