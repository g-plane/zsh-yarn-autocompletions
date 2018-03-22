function _yarn_autocompletions_scripts () {
    compls=$($ZSH_CUSTOM/plugins/yarn-autocompletions/yarn-autocompletions scripts)
    completions=(${=compls})
    compadd -- $completions
}

function _yarn_autocompletions_add () {
    compls=$($ZSH_CUSTOM/plugins/yarn-autocompletions/yarn-autocompletions add)
    completions=(${=compls})
    compadd -- $completions
}

function _yarn_autocompletions_add_dev () {
    compls=$($ZSH_CUSTOM/plugins/yarn-autocompletions/yarn-autocompletions add-dev)
    completions=(${=compls})
    compadd -- $completions
}

function _yarn_autocompletions_remove () {
    compls=$($ZSH_CUSTOM/plugins/yarn-autocompletions/yarn-autocompletions remove)
    completions=(${=compls})
    compadd -- $completions
}

function _yarn_autocompletions () {
    case $words[2] in
        add)
            if [[ $words[3] == "--dev" ]]
            then
                _yarn_autocompletions_add_dev
            else
                _yarn_autocompletions_add
            fi
            ;;
        remove)
            _yarn_autocompletions_remove
            ;;
        *)
            _yarn_autocompletions_scripts
            ;;
    esac
}

compdef _yarn_autocompletions yarn
