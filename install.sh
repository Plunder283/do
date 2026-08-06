cargo build --release
clear
mkdir -p ~/.local/bin/do
mv ./target/release/do ~/.local/bin/do
echo "Installed at ~/.local/bin/do"
