# notechain

A blockchain-backed note taking application written in *rust*.

# Rust major mode for emacs

At least in *emacs* 28 or if [melpa](https://melpa.org/ 
"Milkypostman’s Emacs Lisp Package Archive") is enabled :

	<M-x> package-install <RET>
	rust-mode <RET>
	
# Building on Manjaro

This project uses the default rust build management system : 
[cargo](https://doc.rust-lang.org/cargo/). To install it on arch/manjaro and 
buid project, run these commands :

	sudo pacman -Sy rust
	cargo build

First build may be very long due to (300+) dependencies.

# Running

You have to run the application and set the `RUST_LOG` environment variable :

	RUST_LOG=info cargo run

# Unit tests

This program is shipped with unit tests. To build and execute them, run :

	cargo test

# Commands

Actually, the program prompt accepts these commands :

	ls p            Print discovered peers.
	ls c            Print the genesis block.
	create b <data> Create a new block with the given data.

# API documentation

I try to keep up to date the API documentation. You can generate and view
it in your system browser using :

	cargo doc --open
	
	
