fn main() {
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(r);

    r = &b;
    let is_letter = r.is_ascii();
    dbg!(is_letter);
    dbg!(r);
}