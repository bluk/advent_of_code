pub mod amp;
pub mod error;
pub mod hull_robot;
pub mod intcode;

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::intcode::*;
    use std::io;

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub(crate) struct TestInput {
        pub(crate) input: Vec<String>,
    }

    impl TestInput {
        pub(crate) fn new(mut input: Vec<String>) -> Self {
            input.reverse();
            TestInput { input }
        }
    }

    impl ProgInput for TestInput {
        fn read(&mut self) -> Result<String, Error> {
            if let Some(input) = self.input.pop() {
                Ok(input)
            } else {
                Err(Error::IoErr(io::Error::from(io::ErrorKind::UnexpectedEof)))
            }
        }
    }

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub(crate) struct TestOutput {
        pub(crate) output: Vec<String>,
    }

    impl TestOutput {
        pub(crate) fn new() -> Self {
            TestOutput { output: Vec::new() }
        }
    }

    impl ProgOutput for TestOutput {
        fn write(&mut self, output: &str) -> Result<(), Error> {
            self.output.push(output.to_string());
            Ok(())
        }
    }
}
