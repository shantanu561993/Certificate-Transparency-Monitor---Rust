# Certificate Transparency Monitor

This project is an attempt to monitor certificate transparency logs with purpose of bug bounty or bug hunting. The idea is to monitor the logs with bunch of domain wildcards which are generally found in the Bug Bounty Platforms

## Installation

You can download the release or build your own. To build run 

```bash
cargo build --release 
```

## Usage

Just run the binary. Currently the binary monitors the wildcards listed here 
```
https://github.com/arkadiyt/bounty-targets-data/blob/main/data/wildcards.txt
```
If you want to change the wildcards, just change the line in main.rs 
```rust 
logs_reader::root_reader(Some("<NEW_URL_HERE>".to_string())).await;
```
If you just want to see log/see all certificate transparency domains , modify the main.rs like below 
```rust
async fn main() {

    //logs_reader::root_reader(Some("https://raw.githubusercontent.com/arkadiyt/bounty-targets-data/main/data/wildcards.txt".to_string())).await;
    logs_reader::root_reader(None).await;

}
```
you may need to build the binary after making changes. See Installation on how to build the binary. 

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
