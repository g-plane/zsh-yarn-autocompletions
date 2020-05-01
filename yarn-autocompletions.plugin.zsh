YARN_AUTO_COMP_PATH="$(dirname $0)/yarn-autocompletions"

_yarn_autocompletions() {
    compls=$($YARN_AUTO_COMP_PATH $1)
    completions=(${=compls})
    compadd -- $completions
}

_yarn() {
    case $words[2] in
        add)
            if [[ $words[3] == "--dev" ]]
            then
                _yarn_autocompletions "add-dev"
            else
                _yarn_autocompletions "add"
            fi
            ;;
        remove)
            _yarn_autocompletions "remove"
            ;;
        upgrade)
            _yarn_autocompletions "remove"
            ;;
        why)
            _yarn_autocompletions "why"
            ;;
        *)
            _yarn_autocompletions "scripts"
            ;;
    esac
}

compdef _yarn yarn
