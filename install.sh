cargo build --release
rm -rf $HOME/.local/bin/scrape-icons
ln -s $(pwd)/target/release/font-icons-scraper $HOME/.local/bin/scrape-icons
chmod u+x $HOME/.local/bin/scrape-icons
