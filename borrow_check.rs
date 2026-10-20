fn main() {
    let mut score = 10;
    let lecture = &score;
    println!("{}", lecture);

    let ecriture = &mut score;
    *ecriture = 20;
    println!("{}", ecriture);
}