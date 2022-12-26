/// A command-line prompt handling struct

pub struct Prompt {
     /// The text printed at the start of each line
    pub text: String, 
    pub intro_text: String,
}


impl Prompt {
    pub fn new() -> Self {
        Self { text:       " > ".to_string(),
	       intro_text: "Welcome to notechain v0.0.0-2
use help command to learn more.".to_string()}
    }

    pub fn intro(&self) {
	println!("{}\n", self.intro_text);

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Can instanstiate prompt struct
    #[test]
    fn test_inst() {
	Prompt::new();
    }

    /// Can access the text field
    #[test]
    fn test_text_access() {
	let p = Prompt::new();
	p.text;
    }

    /// Prompt text isn't empty
    #[test]
    fn test_text_not_empty() {
	let p = Prompt::new();
	assert_eq!(p.text.trim().is_empty(), false);
    }

    /// Prompt text isn't empty
    #[test]
    fn test_intro_not_empty() {
	let p = Prompt::new();
	assert_eq!(p.intro_text.trim().is_empty(), false);
    }

   /// Prompt has an callable intro method
    #[test]
    fn test_intro_callable() {
	let p = Prompt::new();
	p.intro();
    }
 
    
}
