use std::collections::VecDeque;
//use std::iter::FromIterator;
    
/// The prompt history
#[derive(Debug)]
pub struct History {
    /// The list of issued commands
    cmds: VecDeque<String>, 
}

impl History {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
	    cmds: VecDeque::new()
	}
    }

    /// Only used in unit tests
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
	return self.cmds.len();
    }

    pub fn add_command(&mut self, command: String)  {
	self.cmds.push_back(command);
    }

    /// Print the current history decque
    #[allow(dead_code)]
    pub fn print(&self) {
	for (idx, cmd) in self.cmds.iter().enumerate() {
	    println!("  {}  {}", idx, cmd);
	}
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Can instanstiate History struct
    #[test]
    fn test_inst() {
	History::new();
    }

    #[test]
    fn test_len() {
	let h = History::new();
	assert_eq!(h.len(), 0);
    }

    #[test]
    fn test_add_command() {
	let mut h = History::new();
	let l1 = h.len();
	h.add_command(String::from("aze"));
	assert_eq!(h.len(), l1 + 1);
    }

    
}
