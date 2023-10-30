ZSH_COMMAND_HISTORY_TOOLDIR=${ZSH_COMMAND_HISTORY_TOOLDIR-${0:A:h}}
HISTORY_BASE_DIR="${XDG_DATA_HOME-$HOME/.local/share}/zsh/CommandHistory/"

function fzf-command-history() {
    local history_local_file
    history_local_file="${HISTORY_BASE_DIR}/${PWD}/.__history"
    read _CURSOR _BUFFER <<< $("${ZSH_COMMAND_HISTORY_TOOLDIR}/rust/command_history/target/release/command_history"  "${history_local_file}")
    if [[ -n "$_CURSOR" ]] && [[ -n "$_BUFFER" ]]; then
        BUFFER="${_BUFFER} "
        CURSOR=${_CURSOR}
        zle redisplay
    fi
}
zle -N fzf-command-history
bindkey "^t" fzf-command-history

function preexec() {
    local history_local_file
    history_local_file="${HISTORY_BASE_DIR}/${PWD}/.__history"
    mkdir -p $(dirname ${history_local_file})
    echo "$1" | sed "s/^/$(date '+%Y-%m-%dT%H:%M:%S%z')\t/" >> ${history_local_file}
}
