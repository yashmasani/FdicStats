#!/bin/sh
set -xe

cargo run

cp statistics/fdic.json www/

rustup target add wasm32-unknown-unknown

#add wget fdic
INSTALLS=$(cargo install --list)
echo ${INSTALLS}

if [ -z "$( echo ${INSTALLS} | grep wasm-pack)" ]
then
  cargo install wasm-pack 
fi


wasm-pack build
cd www
npm install
npm run dev
