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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Can instanstiate History struct
    #[test]
    fn test_inst() {
	History::new();
    }
}
