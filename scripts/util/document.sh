#!/bin/bash

cd $ARC_DIR;
rm -r docs;
cargo doc --document-private-items;
mv target/doc docs;
echo "<head> <meta http-equiv='refresh' content='0; URL=https://freddywordingham.github.io/arc/arc/index.html'/> </head>" >> docs/index.html;
cd -;
