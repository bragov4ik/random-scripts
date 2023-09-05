#!/bin/bash

# get env
script_full_path=$(dirname "$0")
source $script_full_path/settings.sh

echo "Compiling the code on remote"
ssh -t $REMOTE -- "/bin/zsh --login -c \"cd $REMOTE_FOLDER && cargo build --release --features wip,ready-to-test,private-net\""
echo "Launching the private net"
# forward WS port to connect polkadot.js + run the private net
ssh -t -L 127.0.0.1:9945:127.0.0.1:9944 $REMOTE -- "/bin/zsh --login -c \"cd $REMOTE_FOLDER && ./run_script.sh -d -w -r\""
