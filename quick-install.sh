#!/bin/bash
set -e
mkdir maidono-quick-install
cd maidono-quick-install
curl -L -o maidono.tgz https://github.com/louisdevie/maidono/releases/download/v0.1.0/maidono-0.1.0-x86_64-unknown-linux-gnu.tar.gz
tar -xzf maidono.tgz
chmod u+x install.sh
set +e
. install.sh
set -e
cd ..
rm -r maidono-quick-install

