/// A command-line prompt handling struct

use libp2p::{
    swarm::Swarm,
};

use super::*;

/// The String parameter should be the complete command, maybe to handle params
type Callback = fn(String, swarm: &Swarm<p2p::AppBehaviour>);

/// The full prompt object used to handle user input. You should have only one.
pub struct Prompt {
    /// The text printed at the start of each line
    #[allow(dead_code)]
    pub text: String, 
    pub intro_text: String,
    pub help_text: String,
    pub commands: Vec<PromptCommand>,
}

/// A command you can add to the Prompt struct
pub struct PromptCommand {
    /// The callback executed when the starts_width text is found
    pub callback: Callback,
    /// The text that trigger this command if it starts with this text
    pub starts_with: String,
    /// The help text associated with this command
    pub help_text: String,
    /// A possible param text. Please note that the `create b' command do not
    /// use this variable to print the `<data>' param in help text.
    pub param: String,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
	    text:       " > ".to_string(),
	    intro_text: "Welcome to notechain v0.0.0-9
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
	    if cmd.param.is_empty() {
		println!("  {:15} {}", cmd.starts_with, cmd.help_text);
	    }
	    else
	    {
		println!("  {:4} {:10} {}", cmd.starts_with, cmd.param,
			 cmd.help_text);

	    }
	}
	println!("");
    }

    pub fn add(&mut self, pc: PromptCommand){
	self.commands.push(pc);
    }

    pub fn exec(&mut self, cmdstring: String,
		swarm: &Swarm<p2p::AppBehaviour>) -> bool{
	for cmd in self.commands.iter() {
	    //println!("{}\n", cmd.starts_with);
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
	    param: "".to_string(),
	}
    }

    pub fn execute(&self, str: String, swarm: &Swarm<p2p::AppBehaviour>) {
	(self.callback)(str, swarm);
    }
}

#[cfg(test)]
mod tests {
    use libp2p::{
	mdns::Mdns,
	NetworkBehaviour,
	floodsub::{Floodsub, FloodsubEvent},
	mdns::MdnsEvent,
	swarm::NetworkBehaviourEventProcess,
    };
    
    use std::{
	future::Future,
	task::Poll,
	task::Context,
	pin::Pin,
    };

    use crate::p2p::{
	BLOCK_TOPIC,
	CHAIN_TOPIC,
	PEER_ID,
	ChainResponse,
    };
    
    use super::*;

    #[derive(NetworkBehaviour)]
    pub struct TestableBehaviour {
	pub floodsub: Floodsub,
	pub mdns: Mdns,
	#[behaviour(ignore)]
	pub response_sender: mpsc::UnboundedSender<ChainResponse>,
	#[behaviour(ignore)]
	pub init_sender: mpsc::UnboundedSender<bool>,
	#[behaviour(ignore)]
	pub app: App,
    }

    impl TestableBehaviour {
	pub async fn new(
            response_sender: mpsc::UnboundedSender<ChainResponse>,
            init_sender: mpsc::UnboundedSender<bool>,
	) -> Self {
            let mut behaviour = Self {
		app: App::new(),
		floodsub: Floodsub::new(*PEER_ID),
		mdns: Mdns::new(Default::default())
                    .await
                    .expect("can create mdns"),
		response_sender,
		init_sender,
            };
            behaviour.floodsub.subscribe(CHAIN_TOPIC.clone());
            behaviour.floodsub.subscribe(BLOCK_TOPIC.clone());
	    
            behaviour
	}
    }

    impl NetworkBehaviourEventProcess<MdnsEvent> for TestableBehaviour {
	fn inject_event(&mut self, _event: MdnsEvent) {
	    // Nothing
	}
    }

    impl NetworkBehaviourEventProcess<FloodsubEvent> for TestableBehaviour {
	fn inject_event(&mut self, _event: FloodsubEvent) {
	    // Nothing
	}
    }

    /// Trying to implement Future::poll to fix unit tests building issue
    impl Future for TestableBehaviour {
	type Output = ();
	fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>)
		-> Poll<Self::Output> {
	    Poll::Ready(())
	}
    }
    /*
    impl Future<Output = TestableBehaviour> for TestableBehaviour {

    }
  */  
    /* or */ /*
    impl<T> Future for TestableBehaviour<T> {
	
    }
    */
    
    fn get_fake_swarm() -> &'static Swarm<p2p::AppBehaviour> {
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
	let behaviour = TestableBehaviour::new(response_sender,
					       init_sender.clone());

	let mut swarm = SwarmBuilder::new(transp, behaviour, *p2p::PEER_ID)
        .executor(Box::new(|fut| {
            spawn(fut);
        })).build();
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

	//	let swarm = get_fake_swarm();
	
	// Should return true (starts_with found)
	assert_eq!(p.exec("aze".to_string(), get_fake_swarm()), true);
    }
    
}
