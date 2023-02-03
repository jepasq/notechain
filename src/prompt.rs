/// A command-line prompt handling struct

type Callback = fn(String);

/// The full prompt object used to handle user input. You should have only one.
pub struct Prompt {
     /// The text printed at the start of each line
    pub text: String, 
    pub intro_text: String,
    pub help_text: String,
    pub commands: Vec<PromptCommand>,
}

/// A command you can add to the Prompt struct
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
	    commands: Vec::new()
	}
    }

    pub fn intro(&self) {
	println!("{}\n", self.intro_text);
    }

    pub fn help(&self) {
	println!("{}\n", self.help_text);
    }

    pub fn add(&mut self, pc: PromptCommand){
	self.commands.push(pc);
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
	let _pc = PromptCommand::new();
    }

   /// Prompt has an callable intro method
    #[test]
    fn test_prompt_can_access_commands() {
	let p = Prompt::new();
	let _vec = p.commands;
    }

   /// Prompt has an callable intro method
    #[test]
    fn test_prompt_commands_vec_is_empty() {
	let p = Prompt::new();
	let vec = p.commands;
	assert_eq!(vec.is_empty(), true);
    }

   /// Prompt has an callable intro method
    #[test]
    fn test_prompt_has_an_add_method() {
	let mut p = Prompt::new();
	let vecl1 = p.commands.len();

	let pc = PromptCommand::new();
	p.add(pc);
	
	assert_eq!(p.commands.len(), vecl1 + 1);
	
    }
    
}
