if [ -z "$1" ]
then
    echo "Plugins directory not specified, please pass your zsh plugins directory as a parameter."
    exit 1
fi

if [ ! -d $1/yarn-autocompletions ]
then
    mkdir $1/yarn-autocompletions
fi

cp ./yarn-autocompletions.plugin.zsh $1/yarn-autocompletions
cp ./yarn-autocompletions $1/yarn-autocompletions
