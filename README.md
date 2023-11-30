## catlooping

**its just paper backup**

**prerequisites**
```
sudo apt install imagemagick-6.q16hdri
```

**how to get**
```
git clone https://gitlab.com/morthy461/paper_backup.git && \
cd paper_backup && mkdir -p qrcode 
```

```
cargo build
cargo run -- --help
```

**Usage :**
```
usage: paper_backup [option]

option: 
       --help           :  Help command!
       --eff            :  Generate Eff random wordlist
       --eff-lock       :  Generate paper backup with Eff random wordlist
       --diceware       :  Generate passphrase using diceware crate
       --diceware-lock  :  Generate qrcoode paper backup with --diceware
       --mnemonic       :  Generate passphrase using tiny-bip39 crate
       --mnemonic-lock  :  Generate qrcode paper backup using tiny-bip39 crate
       --unlock         :  Unlock qrcode from directory qrcode/
       --lock-string    :  Generate qrcode paper backup from string input
       --qrcode-no-pgp  :  Generate qrcode only no pgp
       --convert        :  Convertion string to ?
```
