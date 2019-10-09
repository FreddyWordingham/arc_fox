#!/bin/bash

cd $ARC_DIR;
sh scripts/document.sh;
git add .;
git commit -m "Updated documentation.";
git push;
cd -;
