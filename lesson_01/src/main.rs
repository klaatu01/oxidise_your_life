// reads a single line from stdin
fn read_single_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().into()
}

// should print "hello ${name}"
fn hello(name: String) -> String {
    unimplemented!()
}

fn main() {
    println!("Enter a name:");

    let name = read_single_line();

    let greeting = hello(name);
    println!("{greeting}");

    // Implement "goodbye" and then uncomment this. What happens?
    // let not_a_greeting = goodbye(name);
    // println!("{not_a_greeting}");
}

#[cfg(test)]
mod test {
    use crate::hello;

    #[test]
    fn hello_test() {
        let input = "ferris";

        let output = hello(input.to_string());

        assert_eq!(output, "hello ferris");
    }

    // #[test]
    // fn goodbye_test() {
    //     let input = "ferris";

    //     let output = goodbye(input.to_string());

    //     assert_eq!(output, "goodbye ferris");
    // }
}
