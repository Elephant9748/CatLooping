## catlooping

**its just paper backup**

**how to get**
```
git clone https://github.com/BigOrt/CatLooping.git && cd CatLooping
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
