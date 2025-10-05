## paperbackup

**its just paper backup**

**prerequisites:**
```
- imagemagick (package manager)
- diceware (cargo install diceware)
- wl-clipboard 

- must have directory in current program running:
    * qrcode/
    * stegano/
```

**how to get**
```
git clone [ this-repo ] paperbackup && \
cd paperbackup && mkdir -p qrcode 
```

```
cargo build
cargo run -- --help
```

**build binaries**
```
cargo build --release
```
located in `~/target/release/paperbackup`

**Usage :**
```
usage: paperbackup [option]

option: 
        --help           :  Help command!
        --version        :  version
        --eff            :  Generate Eff random wordlist
        --eff-lock       :  Generate paper backup with Eff random wordlist
        --diceware       :  Generate passphrase using diceware crate
        --diceware-lock  :  Generate qrcoode paper backup with --diceware
        --mnemonic       :  Generate passphrase using tiny-bip39 crate
        --mnemonic-lock  :  Generate qrcode paper backup using tiny-bip39 crate
        --unlock         :  Unlock qrcode from directory qrcode/
        --lock-string    :  Generate qrcode paper backup from string input
        --qrcode-no-pgp  :  Generate qrcode only no pgp
        --from-file-pgp  :  Generate qrcode with pgp from file
        --from-file      :  Generate qrcode only no pgp from file
        --convert        :  Convertion string to ?
        --entropy-check  :  Check entropy value of password / string
        --password       :  Password generator not include Extended ASCII
        --encode-image   :  Encode message to image
        --decode-image   :  Decode message to image\n
```
