ZSH_COMMAND_HISTORY_TOOLDIR=${ZSH_COMMAND_HISTORY_TOOLDIR-${0:A:h}}
ZSH_COMMAND_HISTORY_BASE_DIR="${XDG_DATA_HOME-$HOME/.local/share}/zsh/CommandHistory"
ZSH_COMMAND_HISTORY_FILE="${ZSH_COMMAND_HISTORY_BASE_DIR}/.__history"

function fzf-command-history() {
    read _CURSOR _BUFFER <<< $( \
        "${ZSH_COMMAND_HISTORY_TOOLDIR}/rust/command_history/target/release/command_history" \
        "${ZSH_COMMAND_HISTORY_FILE}" \
        "${LBUFFER}" \
        "${ZSH_COMMAND_HISTORY_TOOLDIR}" \
    )
    if [[ -n "${_CURSOR}" ]] && [[ -n "${_BUFFER}" ]]; then
        BUFFER="${_BUFFER} "
        CURSOR=${_CURSOR}
        zle redisplay
    fi
}
zle -N fzf-command-history

function preexec() {
    date '+%Y-%m-%dT%H:%M:%S%z' | \
        paste - <(echo "\"${PWD}\"") | \
        paste - <(echo "$1" | perl -pe 'chomp if eof' | tr '\n' '') >> "${ZSH_COMMAND_HISTORY_FILE}"
}
