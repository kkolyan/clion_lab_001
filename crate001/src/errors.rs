pub struct ErrorChain {
    chain: Vec<String>,
}

impl ErrorChain {
    pub fn new<S: Into<String>>(s: S) -> Self {
        ErrorChain {
            chain: vec![s.into()],
        }
    }

    pub fn print_error(&self) {
        println!("ERROR:");
        for link in self.chain.iter() {
            println!("  - {}", link)
        }
    }
}

pub trait ChainableResult: Sized {
    type ResultType;
    fn chain_err<S: Into<String>, F: FnOnce() -> S>(self, f: F) -> Self::ResultType;
}

pub type ChainedResult<T = ()> = Result<T, ErrorChain>;

pub trait ChainableError: Sized {
    fn chain_me<S: Into<String>, F: FnOnce() -> S>(mut self, f: F) -> Self {
        self.get_chain().chain.push(f().into());
        self
    }

    fn get_chain(&mut self) -> &mut ErrorChain;
}

impl <T> ChainableError for (T, ErrorChain) {
    fn get_chain(&mut self) -> &mut ErrorChain {
        &mut self.1
    }
}

impl<T, E: ChainableError> ChainableResult for Result<T, E> {
    type ResultType = Result<T, E>;

    fn chain_err<S: Into<String>, F: FnOnce() -> S>(self, f: F) -> Self::ResultType {
        self.map_err(|err| err.chain_me(f))
    }
}

impl ChainableError for ErrorChain {
    fn chain_me<S: Into<String>, F: FnOnce() -> S>(mut self, f: F) -> Self {
        self.chain.push(f().into());
        self
    }

    fn get_chain(&mut self) -> &mut ErrorChain {
        self
    }
}
