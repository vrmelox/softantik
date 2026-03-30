mod gates;

fn main() {
    let a : bool = false;
    let result = gates::not(a);
    println!("Le résultat {}", result);
}