YARN_AUTO_COMP_PATH="$(dirname $0)/yarn-autocompletions"

_fetch_yarn_autocompletions_result() {
    compls=$($YARN_AUTO_COMP_PATH $1)
    completions=(${=compls})
    compadd -- $completions
}

_yarn_autocompletions() {
    case $words[2] in
        add)
            if [[ $words[3] == "--dev" ]]
            then
                _fetch_yarn_autocompletions_result "add-dev"
            else
                _fetch_yarn_autocompletions_result "add"
            fi
            ;;
        remove)
            _fetch_yarn_autocompletions_result "remove"
            ;;
        upgrade)
            _fetch_yarn_autocompletions_result "remove"
            ;;
        why)
            _fetch_yarn_autocompletions_result "why"
            ;;
        *)
            _fetch_yarn_autocompletions_result "scripts"
            ;;
    esac
}

compdef _yarn_autocompletions yarn
