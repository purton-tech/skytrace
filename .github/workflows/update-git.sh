#!/bin/bash
git add .
git commit -am "chore(deployment): Setting sha256 hashes for $1 [ci skip]"
git push