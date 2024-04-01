use std::collections::VecDeque;
//use std::iter::FromIterator;
    
/// The prompt history
pub struct History {
    /// The list of issued commands
    cmds: VecDeque<String>, 
}

impl History {
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

    /// Print the current history decque
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

}
