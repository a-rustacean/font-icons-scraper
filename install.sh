cargo build --release
rm -rf $HOME/.local/bin/scrap-icons
ln -s $(pwd)/target/release/font-icons-scraper $HOME/.local/bin/scrap-icons
chmod u+x $HOME/.local/bin/scrap-icons
