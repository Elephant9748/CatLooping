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

