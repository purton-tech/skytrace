# Git aliases.
alias gst='git status'
alias gcm='git checkout main'
alias c=clear
alias gp='git push'
alias gcam='git commit -a -m'
alias gpsup="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
alias gcb='git checkout -b'
alias gcr='f() { git checkout -b $1 origin/$1; }; f'
alias gitsetup='git config --global user.name \$NAME && git config --global user.email \$EMAIL'

# Watch
alias watch-app='mold -run cargo watch --workdir /workspace/ -w crates/primer-rsx -w crates/ui-components -w crates/grpc-api -w crates/axum-server -w crates/db -w crates/asset-pipeline/dist -w crates/asset-pipeline/images --no-gitignore -x "run --bin trace"'
alias wa=watch-app
alias watch-pipeline='npm run start --prefix /workspace/crates/asset-pipeline'
alias wp=watch-pipeline
alias watch-zola='cd /workspace/www && zola serve --interface 0.0.0.0 --port 7404'
alias wz=watch-zola

alias pk='export ECDH_PRIVATE_KEY=$(openssl pkcs8 -topk8 -nocrypt -in /workspace/cloak-pulumi.enc.pem)'
alias ck='export ECDH_PRIVATE_KEY=$(openssl pkcs8 -topk8 -nocrypt -in /workspace/cloak.enc.pem)'

# Database
alias db='psql $DATABASE_URL'
alias dbmate='dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations'

alias spell='docker run --rm -ti -v $HOST_PROJECT_PATH/www/content:/workdir tmaier/markdown-spellcheck:latest "**/*.md"'

# Leave a line below or the files will cat together
