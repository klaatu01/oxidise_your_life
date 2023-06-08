// reads a single line from stdin
fn read_single_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().into()
}

// should print "hello ${name}"
fn hello(name: String) {
    unimplemented!()
}

fn main() {
    println!("Enter a name:");

    let name = read_single_line();

    hello(name);
    // Implement goodbye and then uncomment this. What happens?
    // goodbye(name);
}
