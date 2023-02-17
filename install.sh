CP() {
    mkdir -p $(dirname "$2") && cp "$1" "$2"
}

cargo build -r
CP target/release/md-to-html $HOME/.md-to-html/md-to-html
cp css/ -R $HOME/.md-to-html/css

echo ""
echo "--------------------"
echo "md-to-html builded and copied to $HOME/.md-to-html Please add the following code to your .bashrc:"
echo ""
echo "export PATH=\"\$HOME/.md-to-html/:\$PATH\""
echo "--------------------"
echo ""