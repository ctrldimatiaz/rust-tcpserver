# Key-Value TCP Server and Client

## Description 

The purpose of the TCP Server is to perform key-value pairs storage while processing multiple connections commands. More specifically we will have SET, GET and DELETE commands.

## Architecture

`/client` - Here will sit the client responsible for openning connection and sending the requested commands.

`/server` - Here will sit the server responsible for listening in an address, receiving the commands and processing it. <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/server/network` - Responsible for listening, accepting connections and processing the commands <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/server/storage` - Responsible for data storage. Later we will store in on a JSON. 


## Installation

Make sure you have Rust Compiler and Cargo installed. Once installed, you may run the project with --package argument like:

```shell
cargo run -p {package}
```

Example:
```shell
cargo run -p server
```
