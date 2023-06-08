fn alphabet() -> String {
    "ABCDEFGH".into()
}

fn to_lowercase(input: String) -> String {
    unimplemented!()
}

fn to_char_vec(input: String) -> Vec<char> {
    unimplemented!()
}

fn to_lower_char_vec(input: String) -> Vec<char> {
    unimplemented!()
}

fn to_graph_vec(input: String) -> Vec<String> {
    unimplemented!()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{alphabet, to_char_vec, to_graph_vec, to_lower_char_vec, to_lowercase};

    #[test]
    fn to_lowercase_test() {
        let input = alphabet();
        let output = to_lowercase(input);
        assert_eq!(output, "abcdefgh");
    }

    #[test]
    fn to_char_vec_test() {
        let input = alphabet();
        let output = to_char_vec(input);
        assert_eq!(output, vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']);
    }

    #[test]
    fn to_lowercase_char_vec_test() {
        let input = alphabet();
        let output = to_lower_char_vec(input);
        assert_eq!(output, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }

    #[test]
    fn to_graph_vec_test() {
        let input = alphabet();
        let output = to_graph_vec(input);
        assert_eq!(
            output,
            vec!["A->B", "B->C", "C->D", "D->E", "E->F", "F->G", "G->H"]
        );
    }
}
