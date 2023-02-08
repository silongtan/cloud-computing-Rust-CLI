# Rust based shell
A rust based command-line shell. It simulates some functions of linux shell. 

# Flow graph


# Usage
To run the tool use the following code: \
``` cargo run ``` \
Then type the following commands \
``` size ``` \
which list the size of the current directory in bytes \
``` list ``` \
which list all the files under current directory
``` exit ``` \
which exit the shell

## Containerized
``` docker build -t rust-shell . ```
``` docker run --rm -it rust-shell --help ```


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)
