fn main(){
    // Hi
    println!("Hi!");

    /*
        good
     */
    /*
     * Also good
     */
    let x = 5 + /* 90 + */ 10;
    println!("Is 'x' 15 or 105? x = {}", x);

    println!("{} days", 30);
    println!("{0}, {1}", "GoGo", "BaBa");
    println!("{cool} {is} {cold}",
            cool="Cool",
            is="Is",
            cold="Cold",);
    println!("B10 {}", 256256);
    println!("B2 {:b}", 256256);
    println!("B8 {:o}", 256256);
    println!("B16 {:x}", 256256);

    println!("{number:>5}", number=2);
    println!("{number:0>5}", number=2);
    println!("{number:0<5}", number=2);
    println!("{number:0>width$}", number=2, width=7);

    
    let number: f64 = 1.0;
    let width: usize = 10;
    println!("{number:a>width$}");
    
    /*
    Activities
    
    1. Fix the issue in the above code (see FIXME) so that it runs without error.
    2. Try uncommenting the line that attempts to format the Structure struct (see TODO)
    3. Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
    
    */
    
    println!("0{0} 1{1} 0{0}", "GoGi", "James");
    // 1. FIXME ^ Add the missing argument: "James"
    

    #[allow(dead_code)]
    struct Structure(i32);
    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // 2. TODO ^ Try uncommenting this line

    let pi = 3.141592;
    println!("{:.3}", pi);
    // 3. ^
}