//! This rust crate implement `notechain` : A blochain-backed note taking
//! application.
//!
//! While the persistence and multi-user approach is handled using a
//! blockchain, the applicarion itself is currently based on a CLI interface.
use chrono::Utc;
use sha2::Sha256;
use sha2::Digest;

use log::{info, warn, error};  // USES warn!, error! etc...
use serde::{Deserialize, Serialize};

use std::time::Duration;

use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    select, spawn,
    sync::mpsc,
    time::sleep,
};

use std::process;    // USES exit()

const DIFFICULTY_PREFIX: &str = "00";

mod p2p;
mod prompt;
mod history;

const GENHASH:
    &str = "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce,hash)=mine_block(id, now.timestamp(),&previous_hash, &data);
        Self {
            id,
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce,
        }
    }
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining block...");
    let mut nonce = 0;

    loop {
        if nonce % 100000 == 0 {
            info!("nonce: {}", nonce);
        }
        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary_representation(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            info!(
                "mined! nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

#[derive(Debug)]
pub struct App {
    pub blocks: Vec<Block>,
    pub history: history::History,
}

impl App {
    fn new() -> Self {
        Self {
	    blocks: vec![],
	    history: history::History::new()
	}
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: GENHASH.to_string(),
        };
        self.blocks.push(genesis_block);
    }
    
    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last()
	    .expect("there is at least one block");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("could not add block - invalid");
        }
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"),
        )
        .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }

    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !self.is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    // We always choose the longest valid chain
    fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>)
		    -> Vec<Block> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("local and remote chains are both invalid");
        }
    }
}

/// A very simple default callback
pub fn quit_callback(_cmdtext: String, _swarm: &Swarm<p2p::AppBehaviour>) {
    println!("Exitting application...");
    // from https://doc.rust-lang.org/std/process/fn.exit.html
    process::exit(0x0100);
}

pub fn len_callback(_cmdtext: String, swarm: &Swarm<p2p::AppBehaviour>) {
    let len = &swarm.behaviour().app.blocks.len();
    println!("Actual blockchain length in blocks : {}", len);
}

pub fn hist_callback(_cmdtext: String, swarm: &Swarm<p2p::AppBehaviour>) {
    let len = &swarm.behaviour().app.history.len();
    println!("History length : {}\n", len);
    swarm.behaviour().app.history.print();
}

pub fn nth_callback(cmdtext: String, swarm: &Swarm<p2p::AppBehaviour>) {
    let hst = &swarm.behaviour().app.history;
    let mut s = cmdtext.clone();
    if s.len() > 0 {
	s.remove(0); // remove first (should be '!' char)
    }
    let nth = s.parse::<i32>().unwrap();
    println!("Calling {}th command : {}\n", nth, "aze");
    swarm.behaviour().app.history.print();
}



#[tokio::main]
async fn main() {
    // Setup
    pretty_env_logger::init();

    let mut pr = prompt::Prompt::new();
    pr.intro();

    // Create and add the len command
    let mut len_cmd = prompt::PromptCommand::new();
    len_cmd.starts_with= "len".to_string();
    len_cmd.help_text= "print the blockchain length in blocks"
	.to_string();
    len_cmd.callback = len_callback;
    pr.add(len_cmd);

    // Create and add the quit command
    let mut hist_cmd = prompt::PromptCommand::new();
    hist_cmd.starts_with= "hist".to_string();
    hist_cmd.help_text= "print the last used commands"
	.to_string();
    hist_cmd.callback = hist_callback;
    pr.add(hist_cmd);

    // Create and add the positional arg command
    let mut nth_cmd = prompt::PromptCommand::new();
    nth_cmd.starts_with= "!".to_string();
    nth_cmd.help_text= "re-execute nth command from history"
	.to_string();
    nth_cmd.callback = nth_callback;
    pr.add(nth_cmd);
    
    // Create and add the quit command
    let mut quit_cmd = prompt::PromptCommand::new();
    quit_cmd.starts_with= "quit".to_string();
    quit_cmd.help_text= "exit the program with a success status code"
	.to_string();
    quit_cmd.callback = quit_callback;
    pr.add(quit_cmd);

    
    info!("Peer Id: {}", p2p::PEER_ID.clone());
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

    let behaviour = p2p::AppBehaviour::new(App::new(), response_sender, init_sender.clone()).await;

    let mut swarm = SwarmBuilder::new(transp, behaviour, *p2p::PEER_ID)
        .executor(Box::new(|fut| {
            spawn(fut);
        }))
        .build();

    let mut stdin = BufReader::new(stdin()).lines();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can get a local socket"),
    )
    .expect("swarm can be started");

    spawn(async move {
        sleep(Duration::from_secs(1)).await;
        info!("sending init event");
        init_sender.send(true).expect("can send init event");
    });

    // main loop
    loop {
        let evt = {
            select! {
                line = stdin.next_line() => Some(p2p::EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                response = response_rcv.recv() => {
                    Some(p2p::EventType::LocalChainResponse(response.expect("response exists")))
                },
                _init = init_rcv.recv() => {
                    Some(p2p::EventType::Init)
                }
                event = swarm.select_next_some() => {
                    info!("Unhandled Swarm Event: {:?}", event);
                    None
                },
            }
        };

        if let Some(event) = evt {
            match event {
                p2p::EventType::Init => {
                    let peers = p2p::get_list_peers(&swarm);
                    swarm.behaviour_mut().app.genesis();

                    info!("connected nodes: {}", peers.len());
                    if !peers.is_empty() {
                        let req = p2p::LocalChainRequest {
                            from_peer_id: peers
                                .iter()
                                .last()
                                .expect("at least one peer")
                                .to_string(),
                        };

                        let json = serde_json::to_string(&req).expect("can jsonify request");
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(p2p::CHAIN_TOPIC.clone(), json.as_bytes());
                    }
                }
                p2p::EventType::LocalChainResponse(resp) => {
                    let json = serde_json::to_string(&resp)
			.expect("can jsonify response");
                    swarm.behaviour_mut()
                        .floodsub
                        .publish(p2p::CHAIN_TOPIC.clone(), json.as_bytes());
                }
                p2p::EventType::Input(line) => match line.as_str() {
                    "ls p" => p2p::handle_print_peers(&swarm),
		    cmd if cmd.starts_with("hel") => {
			swarm.behaviour_mut().app.history.add_command(line);
			pr.help()
		    },
                    cmd if cmd.starts_with("ls c") => {
			swarm.behaviour_mut().app.history.add_command(line);
			p2p::handle_print_chain(&swarm)
		    },
                    cmd if cmd.starts_with("create b") => {
			swarm.behaviour_mut().app.history
			    .add_command(line.clone());
			p2p::handle_create_block(cmd, &mut swarm)
		    },
		    _ =>  {
			pr.exec_noret(line.clone(), &swarm);
			swarm.behaviour_mut().app.history.add_command(line)
		    }
			
                },
            }
        }
    }
}
