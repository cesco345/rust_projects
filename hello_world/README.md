$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

To check whether you have Rust installed correctly, open a shell and enter this line:
$ rustc --version

Add Rust to Your PATH:
nano ~/.zshrc   or for bash: nano ~/.bashrc
After installation, restart your terminal or run:
source $HOME/.cargo/env
Verify the installation by checking the version:
rustc --version
rustc 1.83.0 (90b35a623 2024-11-26)

test it with hello_world program
++++++++++++++++++++++++++++++++

$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world

nano main.rs
Filename: main.rs

fn main() {
    println!("Hello, world!");
}


$ rustc main.rs
$ ./main
Hello, world!
