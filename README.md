# pov
Porte Occulaire VMA

Initialisation :
==============

cd cargo

cargo new pov

cd pov

git init

git config --list --show-origin

git config --global user.name "PatrickRioche/pov"

git config --global user.email "patrick.rioche@gmail.com"

git add .

git remote add origin https://github.com/PatrickRioche/pov.git

git commit -m "init"

git push -u origin master

verification :
=============

git branch -a

git branch devops

git checkout master ou devops
