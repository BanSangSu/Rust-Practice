// Modules https://google.github.io/comprehensive-rust/modules/modules.html
fn module() {
    mod foo {
        pub fn do_something() {
            println!("In the foo module");
        }
    }

    mod bar {
        pub fn do_something() {
            println!("In the bar module");
        }
    }

    foo::do_something();
    bar::do_something();

}

fn main() {
    module();
}
