#!/bin/bash

rm -rf extension
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
  sed -i "s|${scripts[url]}|${scripts[file]}|g" index.html
done

zip -r extension.zip *
mv extension.zip ../
