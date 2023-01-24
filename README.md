# nodechain

A blockchain-backed note taking application written in *rust*.

# Rust major mode for emacs

At least in *emacs* 28 or if [melpa](https://melpa.org/ 
"Milkypostman’s Emacs Lisp Package Archive") is enabled :

	<M-x> package-install <RET>
	rust-mode <RET>
	
# Building on Manjaro

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

	ls p            Print discovered peers.
	ls c            Print the genesis block.
	create b <data> Create a new block with the given data.
