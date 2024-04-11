#!/usr/bin/zsh
export PATH=$HOME/bin:/usr/local/bin:$PATH
export ZSH=$HOME/.oh-my-zsh

DISABLE_AUTO_UPDATE=true
DISABLE_UPDATE_PROMPT=true
ZSH_THEME="devcontainers"
COMPLETION_WAITING_DOTS="true"
DISABLE_UNTRACKED_FILES_DIRTY="true"
plugins=(git)
source $ZSH/oh-my-zsh.sh

alias lll="ls -alh"
alias ll="ls -alh"
alias l="ls -alh"
