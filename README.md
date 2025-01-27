## paper_backup

**its just paper backup**

**prerequisites:**
```
- imagemagick (package manager)
- diceware (cargo install diceware)

- must have directory in current program running:
    * qrcode/
    * stegano/
```

**how to get**
```
git clone [ this-repo ] paper_backup && \
cd paper_backup && mkdir -p qrcode 
```

```
cargo build
cargo run -- --help
```

**build binaries**
```
cargo build --release
```
located in `~/target/release/paper_backup`

**Usage :**
```
usage: paper_backup [option]

option: 
       --help           :  Help command!
       --eff            :  Generate Eff random wordlist
       --eff-lock       :  Generate paper backup with Eff random wordlist
       --diceware       :  Generate seed passphrase using diceware crate
       --diceware-lock  :  Generate qrcoode paper backup with --diceware
       --mnemonic       :  Generate seed passphrase using tiny-bip39 crate
       --mnemonic-lock  :  Generate qrcode paper backup using tiny-bip39 crate
       --unlock         :  Unlock qrcode from directory qrcode/
       --lock-string    :  Generate qrcode paper backup from string input
       --qrcode-no-pgp  :  Generate qrcode only no pgp
       --convert        :  Convertion string to ?
       --entropy-check  :  Check entropy value of password / string
       --password       :  Password Generator not include Extended ASCII
```
