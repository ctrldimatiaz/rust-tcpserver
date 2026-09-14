# Key-Value TCP Server and Client

## Description 

The purpose of the TCP Server is to perform key-value pairs storage while processing multiple connections commands. More specifically we will have SET, GET and DELETE commands.

## Architecture

`/client` - Here will sit the client responsible for openning connection and sending the requested commands.

`/server` - Here will sit the server responsible for listening in an address, receiving the commands and processing it. <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/server/network/listener` - Responsible for listening, accepting connections.<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/server/network/handler` - Responsible for processing each command message.<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/server/storage/store` - Responsible for data storage through any available command with get, set and delete methods. 

`/protocol` - Lib responsible for having the commons enums and functions shared across the protocol between client and server. 
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/protocol/src/command` - Command and response enums.<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/protocol/src/errors` - Parsing errors enum.<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/protocol/src/parser` - Here sits the function that parses the commands.

## Installation

Make sure you have Rust Compiler and Cargo installed. Once installed, you may run the project with --package argument like:

```shell
cargo run -p {package}
```
You should have the server running before running the client as follows: 

```shell
cargo run -p server
cargo run -p client
```
## Environment variables

`RUST_LOG`<br/>
`IP`<br/>
`PORT`

`RUST_LOG` defaults to info.<br/>

Available log levels include:<br/>

error (Level 1)<br/>
warn (Level 2)<br/>
info (Level 3)<br/>
debug (Level 4)<br/>
trace (Level 5)

`IP` used across server and client packages<br/>
`PORT` used across server and client packages
