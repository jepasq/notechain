/// A command-line prompt handling struct

type Callback = fn(String);

pub struct Prompt {
     /// The text printed at the start of each line
    pub text: String, 
    pub intro_text: String,
    pub help_text: String,
}

pub struct PromptCommand {
    /// The callback
    callback: Callback,
    /// The text that trigger this command if it starts with this text
    starts_with: String,
    /// The help text
    help_text: String,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
	    text:       " > ".to_string(),
	    intro_text: "Welcome to notechain v0.0.0-2
use help command to learn more.".to_string(),
	    help_text: "Available commands
help   print the text you're actually reading.".to_string(),
	}
    }

    pub fn intro(&self) {
	println!("{}\n", self.intro_text);
    }

    pub fn help(&self) {
	println!("{}\n", self.help_text);
    }
}

/// A very simple default callback
pub fn nyi_callback(cmdtext: String) {
    println!("NYI callback for '{}' command", cmdtext);
}

impl PromptCommand {
    pub fn new() -> Self {
	Self {
	    callback: nyi_callback,
	    starts_with: "".to_string(),
	    help_text: "".to_string(),
	    
	}
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

    /// Prompt text isn't empty
    #[test]
    fn test_helptext_not_empty() {
	let p = Prompt::new();
	assert_eq!(p.help_text.trim().is_empty(), false);
    }
    
   /// Prompt has an callable intro method
    #[test]
    fn test_has_an_help_method() {
	let p = Prompt::new();
	p.help();
    }

   /// PromptCommand has an callable intro method
    #[test]
    fn test_prompt_command_has_a_new_method() {
	let pc = PromptCommand::new();
    }

    
}
