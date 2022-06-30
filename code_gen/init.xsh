#!/usr/bin/env xonsh

p"~/.xonshrc".exists() && source ~/.xonshrc

from os.path import dirname,abspath,basename
from fire import Fire

PWD = dirname(abspath(__file__))
cd @(PWD)

# cargo install fd-find sd

@Fire
def main(git_ssh=None):
  name = basename(PWD)
  rm -rf .git
  sh -c @(f"sd 'rmw_gen' '{name}' $(fd --type file)")
  git init
  git add .
  git commit -minit
  if not git_ssh:
    git_ssh = f"git@github.com:rmw-lib/{name}.git"
  git remote add origin @(git_ssh)
  try:
    git push -f --tag --set-upstream origin master
  except:
    pass

  rm -rf @(__file__)
