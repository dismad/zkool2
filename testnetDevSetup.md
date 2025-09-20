# HOWTO: mine testnet TAZ with Zebra

## Pre-reqs

### rustup 

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Ubuntu

`sudo apt install llvm-dev clang protobuf-compiler libprotobuf-dev screen`


### Compile with internal-miner

`cargo install --git https://github.com/ZcashFoundation/zebra --tag v2.5.0 zebrad --features internal-miner`


### Update config.toml

`nano ~/.config/zebrad.toml`

Add the following to the indicated sections, if missing:

```rust
[network]
network = "Testnet"

[rpc]
listen_addr = "127.0.0.1:18232"

[mining]
miner_address = "your transparent testnet address here"
internal_miner = true 
```

### Sync testnet node

```bash
screen -S TestnetZebra
zebrad start
```

# HOWTO: Compile Zkool with testnet Support


## Pre-reqs


### Ubuntu

`sudo apt install clang cmake git ninja-build pkg-config libgtk-3-dev liblzma-dev libstdc++-12-dev curl unzip xz-utils zip libglu1-mesa`


## VS code

https://code.visualstudio.com/Download

download .deb for your architecture ( most will be x64 )

`sudo dpkg -i code_1.104.1-1758154125_amd64.deb`

note: file name my be slightly different, check what is downloaded

Launch VS Code Quick Open (Ctrl+P), paste the following command, and press enter.

`ext install Dart-Code.flutter`


## flutter

press Control + Shift + P in VS Code

type `flutter`

Select Flutter: New Project.

VS Code prompts you to locate the Flutter SDK on your computer. Select Download SDK.

When the Select Folder for Flutter SDK dialog displays, choose where you want to install Flutter.

Click Clone Flutter.

Click Add SDK to PATH


## Zkool2

### Download files

```bash
git clone https://github.com/dismad/zkool2.git
cd zkool2
git checkout testnet
```

The following was changed in coin.rs

```rust
            let coin_value = if testnet {
                "1"
            } else if regtest {
                "2"
            } else {
                "1"
            };
```

which will force zkool to default to testnet 

### Compile

`flutter config --enable-linux-desktop`

Then

`flutter build linux -v`

### Run

Open zkool and create an account which should display testnet addresses

Configure the Light wallet server to:

`https://testnet.zec.rocks:443'

or localhost if your running your own zebrad + lwd setup



