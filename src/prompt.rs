/// A command-line prompt handling struct
use libp2p::swarm::Swarm;

use super::*;

/// The String parameter should be the complete command, maybe to handle params
type Callback = fn(String, swarm: &Swarm<p2p::AppBehaviour>);

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
    pub callback: Callback,
    /// The text that trigger this command if it starts with this text
    pub starts_with: String,
    /// The help text
    pub help_text: String,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
	    text:       " > ".to_string(),
	    intro_text: "Welcome to notechain v0.0.0-5
use `help' command to learn more.".to_string(),
	    help_text: "Available commands (1st letter may be enough)
  help            print the text you're actually reading.
  create b <data> Create a new block with the given data.
  ls p            Print discovered peers.
  ls c            Print the genesis block.".to_string(),
	    commands: Vec::new()
	}
    }

    pub fn intro(&self) {
	println!("{}\n", self.intro_text);
    }

    pub fn help(&self) {
	println!("\n{}", self.help_text);

	for cmd in self.commands.iter() {
	    println!("  {:15} {}", cmd.starts_with, cmd.help_text);
	}
	println!("");
    }

    pub fn add(&mut self, pc: PromptCommand){
	self.commands.push(pc);
    }

    pub fn exec(&mut self, cmdstring: String,
		swarm: &Swarm<p2p::AppBehaviour>) -> bool{
	for cmd in self.commands.iter() {
	    println!("{}\n", cmd.starts_with);
	    if cmdstring.starts_with(&cmd.starts_with) {
		cmd.execute(cmdstring, swarm);
		return true;
	    }
	}
	return false;
    }

    pub fn exec_noret(&mut self, cmdstring: String,
		      swarm: &Swarm<p2p::AppBehaviour>) {
	let c = cmdstring.clone();
	if !self.exec(cmdstring, swarm) {
	    println!("Unknown command '{}'", c);
	}
    }
}

/// A very simple default callback
pub fn nyi_callback(cmdtext: String, _swarm: &Swarm<p2p::AppBehaviour>) {
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

    pub fn execute(&self, str: String, swarm: &Swarm<p2p::AppBehaviour>) {
	(self.callback)(str, swarm);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fake_swarm() -> &Swarm<p2p::AppBehaviour> {
	let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
	let (init_sender, mut init_rcv) = mpsc::unbounded_channel();

	let auth_keys = Keypair::<X25519Spec>::new()
            .into_authentic(&p2p::KEYS)
            .expect("can create auth keys");

	let transp = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();

	// May add await() at the end if fn is async
	let behaviour = p2p::AppBehaviour::new(App::new(), response_sender, init_sender.clone());

	let mut swarm = SwarmBuilder::new(transp, behaviour, *p2p::PEER_ID)
        .executor(Box::new(|fut| {
            spawn(fut);
        }))
            .build();
	return &swarm;
    }
    
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

   /// Prompt has a callable intro method
    #[test]
    fn test_prompt_has_an_add_method() {
	let mut p = Prompt::new();
	let vecl1 = p.commands.len();

	let pc = PromptCommand::new();
	p.add(pc);
	
	assert_eq!(p.commands.len(), vecl1 + 1);
    }

    /// Prompt has an callable intro method
    #[test]
    fn test_prompt_command_has_an_execute_method() {
	let pc = PromptCommand::new();
	pc.execute("".to_string(), get_fake_swarm());
    }

    /// Prompt has an callable intro method
    #[test]
    fn test_prompt_exec_method_notfound() {
	let mut p = Prompt::new();

	let mut pc = PromptCommand::new();
	pc.starts_with= "azeaze".to_string();
	pc.help_text= "zerzer".to_string();

	// Should return false (not found)
	assert_eq!(p.exec("RRaze".to_string(), get_fake_swarm()), false);
    }
    
    /// Prompt has an callable intro method
    #[test]
    fn test_prompt_exec_method_found_false() {
	let mut p = Prompt::new();

	let mut pc = PromptCommand::new();
	pc.starts_with= "azeaze".to_string();
	pc.help_text= "zerzer".to_string();

	p.add(pc);

	// Should return false
	assert_eq!(p.exec("tut".to_string(), get_fake_swarm()), false);
    }

    /// Prompt has an callable intro method
    #[test]
    fn test_prompt_exec_method_found_true() {
	let mut p = Prompt::new();

	let mut pc = PromptCommand::new();
	pc.starts_with= "azeaze".to_string();
	pc.help_text= "zerzer".to_string();

	p.add(pc);

	let mut swarm = SwarmBuilder::new(transp, behaviour, *p2p::PEER_ID)
        .executor(Box::new(|fut| {
            spawn(fut);
        }))
        .build();
	
	// Should return true (starts_with found)
	assert_eq!(p.exec("aze".to_string(), get_fake_swarm()), true);
    }
    
}
