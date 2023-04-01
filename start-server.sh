#!/bin/sh
set -xe

rustup target add wasm32-unknown-unknown

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
