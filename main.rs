// projeto para estudos de loops for e condicionais

fn main() {
    for i in 1..=20 {
        if i < 10 {
            println!("numero pequeno: {}", i);
        } else if i == 10 {
            println!("chegamos no 10!");
        } else {
            println!("numero grande: {}", i);
        }
    }

    println!("contagem finalizada!")
}