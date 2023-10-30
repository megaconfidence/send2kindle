#!/usr/bin/env bash

#NOTE: if you are on macOS, update to bash v4 i.e brew install bash

rm -rf extension extension.zip
cp -r public extension 
cd extension
 
declare -A scripts0=(
    [file]='plausible.js'
    [url]='https://plausible.confidence.sh/js/script.js'
)
declare -A scripts1=(
    [file]='buttons.js'
    [url]='https://buttons.github.io/buttons.js'
)

declare -n scripts
for scripts  in ${!scripts@}; do
  curl ${scripts[url]} -o ${scripts[file]}
  sed -i"" -e "s|${scripts[url]}|${scripts[file]}|g" index.html
done

zip -r extension.zip *
mv extension.zip ../
