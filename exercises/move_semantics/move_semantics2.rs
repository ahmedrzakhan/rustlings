// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec0 = Vec::new();
    // let copyVec0 = vec0.clone(); // to send clone
    // let mut vec1 = fill_vec(copyVec0);
    // let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1); // not required for mutable borrowing
    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

// with borrowing
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }
// mutably borrow
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
