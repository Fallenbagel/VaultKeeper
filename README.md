# VaultKeeper
VaultKeeper is a command-line tool written in Rust 🦀 to streamline the backup and restoration process for your Vaultwarden instance. Take command of your vaultwarden data with precision, selecting exactly what to back up and how many versions to retain.

I developed this command-line tool as a means of learning Rust 🦀.

### Key Features:
- 🔒 **Customizable Backups:**

  Tailor your backups to meet your specific needs. VaultKeeper lets you choose which elements to include in your backups, such as the database, attachments, sent items, configuration files (config.json), RSA key, and icon caches.
- 🔄 **Version Control:**

  Define how many backups to retain at a time. VaultKeeper makes it easy to manage storage space by automatically handling the removal of older backups based on your specified retention count.

### Upcoming Features:
- 🔄 **Effortless Restoration:**
  
  Restore your Vaultwarden data with ease. VaultKeeper simplifies the restoration process by restoring the vaultwarden data using a single command!
- 🔐 **Encryption for Added Security:**
  
  Encrypt your Vaultwarden backup data with ease, and decrypt them during the restoration process.

## Usage:

```bash
# Generate your vaultkeeper config file during installation (handled by Makefile) 
vaultkeeper generate-config
# Or generate to a custom directory (handled by Makefile)
vaultkeeper generate-config --config /foo

# Backup your Vaultwarden data after configuring Vaultkeeper
vaultkeeper backup
# Or backup using the config file from a custom path
vaultkeeper backup --config /foo/config.json
```

### Getting Started:
To get started with VaultKeeper, follow these steps:
#### Automatic Installation
1. Download the Makefile:
    ```bash
    wget https://raw.githubusercontent.com/Fallenbagel/vaultkeeper/main/Makefile
    ```
2. Run the installation:
    ```bash
    make install
    ```
#### Manual Installation
1. Download and Install:
   - Visit the [releases](https://github.com/Fallenbagel/VaultKeeper/releases/latest/download/vaultkeeper) page.
   - Download the latest version of VaultKeeper for your platform (windows coming soon™)
2. Make the binary executable (Linux/Unix/Mac):
    ```bash
    chmod +x vaultkeeper
    ```
3. Move the binary to a directory in your system's PATH, e.g., /usr/bin/ (Linux/Unix/Mac):
    ```bash
    sudo mv vaultkeeper /usr/bin/
    ```
Now you can use `vaultkeeper` as a command in your terminal and you can use it by running the commands described in the [Usage](#usage) section.

#### Build from Source:
1. **Install Rust:**
   Ensure that you have Rust installed on your system. If not, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

2. **Clone the Repository:**
   ```bash
   git clone https://github.com/Fallenbagel/vaultkeeper.git
   cd vaultkeeper
   ```
3. Build the binary:
   ```bash
   cargo build --release
   ```
4. Copy the Binary: Once the build process is complete, you can find the binary in the target/release/ directory. Copy it to a location in your system's PATH:
   ```bash
   sudo cp target/release/vaultkeeper /usr/bin/
   ```
5. Make the directory to store your Vaultkeeper config file
   ```bash
   sudo mkdir /etc/vaultkeeper
   ```
6. Generate config file
   ```bash
   vaultkeeper generate-config --config /etc/vaultkeeper
   ```
Now you can use it by running the commands described in the [Usage](#usage) section.

## Contribute:
- If you encounter issues or have suggestions, feel free to open an [issue](https://github.com/Fallenbagel/VaultKeeper/issues/new).
- Contributions are welcome!

