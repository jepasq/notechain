/// A command-line prompt handling struct

pub struct Prompt {
     /// The text printed at the start of each line
    pub text: String, 
    
}


impl Prompt {
    pub fn new() -> Self {
        Self { text : " > ".to_string()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Can instanstiate prompt struct
    fn test_inst() {
	Prompt::new();
    }
}
