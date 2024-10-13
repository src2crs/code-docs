#[derive(Debug, PartialEq)]
pub enum Code {
    /// A plain code snippet that won't be parsed further.
    Plain(String),
    /// A block of code that is contained within comments marking the beginning and end of the block.
    FencedBlock {
        doc_label: String,
        content: Vec<Code>,
    },
}

impl Code {
    /// Creates a new plain code snippet from a string.
    pub fn plain<S: Into<String>>(s: S) -> Self {
        Self::Plain(s.into())
    }

    /// Creates a new fenced block of code from a label (like "EXAMPLE, SOLUTION") and a list of code snippets.
    pub fn fenced_block<S: Into<String>>(doc_label: S, content: Vec<Code>) -> Self {
        Self::FencedBlock {
            doc_label: doc_label.into(),
            content,
        }
    }

    /// Converts the code snippet to a string of Go source code.
    pub fn to_go_source(&self) -> String {
        use Code::*;
        match self {
            Plain(c) => c.clone(),
            FencedBlock { doc_label, content } => {
                let begin_doc = format!("// {}", doc_label);
                let end_doc = format!("// {}_END", doc_label);
                let inner_block = content
                    .iter()
                    .map(|c| c.to_go_source())
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("{}\n{}\n{}\n", begin_doc, inner_block, end_doc)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn plain() {
            let code = Code::plain("var n int");
            assert_eq!(code, Code::Plain("var n int".to_string()));
        }

        #[test]
        fn fenced_block() {
            let lines = ["for i := 0; i < 10; i++ {", "    fmt.Println(i)", "}"]
                .iter()
                .map(|s| Code::plain(*s))
                .collect();

            let code = Code::fenced_block("EXAMPLE", lines);

            assert_eq!(
                code,
                Code::FencedBlock {
                    doc_label: "EXAMPLE".to_string(),
                    content: vec![
                        Code::plain("for i := 0; i < 10; i++ {"),
                        Code::plain("    fmt.Println(i)"),
                        Code::plain("}"),
                    ],
                }
            );
        }
    }

    mod to_go_source {
        use super::*;

        #[test]
        fn plain() {
            let code = Code::plain("var n int");
            assert_eq!(code.to_go_source(), "var n int");
        }

        #[test]
        fn fenced_block() {
            let lines = ["for i := 0; i < 10; i++ {", "    fmt.Println(i)", "}"]
                .iter()
                .map(|s| Code::plain(*s))
                .collect();

            let code = Code::FencedBlock {
                doc_label: "EXAMPLE".to_string(),
                content: lines,
            };

            assert_eq!(
                code.to_go_source(),
                "// EXAMPLE\nfor i := 0; i < 10; i++ {\n    fmt.Println(i)\n}\n// EXAMPLE_END\n"
            );
        }
    }
}
