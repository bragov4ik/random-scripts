#!/bin/bash

# get env
script_full_path=$(dirname "$0")
source $script_full_path/settings.sh

echo "Synchronizing repository..."
ssh -t $REMOTE -- mkdir -p $REMOTE_FOLDER
rsync --itemize-changes --recursive --exclude=target --exclude=.cargo --exclude=.idea --exclude=.vscode $LOCAL_FOLDER/ $REMOTE:$REMOTE_FOLDER
# ls $LOCAL_FOLDER | xargs -n1 -P4 -I% rsync --itemize-changes --recursive --exclude=target --exclude=.cargo --exclude=.idea --exclude=.vscode % $REMOTE:$REMOTE_FOLDER
# ls $LOCAL_FOLDER | parallel -v -j8 rsync --itemize-changes --recursive --exclude=target --exclude=.cargo --exclude=.idea --exclude=.vscode {} $REMOTE:$REMOTE_FOLDER/{}
echo "Successfully synchronized the repo!"
