
// Review of Program Memory https://google.github.io/comprehensive-rust/memory-management/review.html
fn review_programme_memory() {
    let s1 = String::from("Helllo");
    println!("{}", s1);

    let mut s2 = String::from("Goten Tag");
    s2.push(' ');
    s2.push_str("world");

    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s2);
        println!("capacity = {capacity}, ptr = {ptr:#x}, len = {len}");
    }
}



fn main() {
    review_programme_memory();
}
