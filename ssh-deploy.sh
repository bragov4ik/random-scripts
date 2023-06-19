#!/bin/bash

REMOTE_FOLDER="~/Documents/RustProjects/sora2-network"
LOCAL_FOLDER="/Users/bragov4ik/Documents/Soramitsu/Code/sora2-network" 
REMOTE="bragov4ik@192.168.88.55"
echo "Synchronizing repository..."
rsync --itemize-changes --recursive --exclude=target --exclude=.cargo --exclude=.idea --exclude=.vscode $LOCAL_FOLDER $REMOTE:$REMOTE_FOLDER/../
echo "Successfully synchronized the repo!"
echo "Compiling the code on remote"
ssh -t $REMOTE -- "/bin/bash --login -c \"cd $REMOTE_FOLDER && cargo build --release --features wip,private-net\""
echo "Launching the private net"
ssh -t $REMOTE -- "/bin/bash --login -c \"cd $REMOTE_FOLDER && ./run_script.sh -d -w -r\""

