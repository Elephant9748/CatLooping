set quiet

# relase with alert
r:
        cargo b -r -j2 && dunstify -u normal -i "/home/rigel/.local/share/icons/rustacean-flat-happy.svg"  "🦀 cargo compilling finished!"
# relase with alert & optimize
r-opt:
        cargo b --profile release-small -j2 && dunstify -u normal -i "/home/rigel/.local/share/icons/rustacean-flat-happy.svg"  "🦀 cargo compilling finished!"
# build with alert
b:
        cargo b -j2 && dunstify -u normal -i "/home/rigel/.local/share/icons/rustacean-flat-happy.svg"  "🦀 cargo compilling finished!"
# clean build
clean:
        cargo clean
#Aur build pkg
aur-build:
        #!/usr/bin/env bash
        cd aur 
        makepkg -cf
#Aur clean after build pkg
aur-clean:
        #!/usr/bin/env bash
        rm -rf aur/paperbackup aur/*.zst aur/pkg aur/src
#Aur install pkg into system
aur-install:
        #!/usr/bin/env bash
        cd aur 
        makepkg -si
# build for nixos
nix:
        nix build . --impure -L

