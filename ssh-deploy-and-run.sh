#!/bin/bash

# get env
script_full_path=$(dirname "$0")
$script_full_path/ssh-deploy-only.sh
$script_full_path/ssh-run-only.sh
