if [ ! -d $1/plugins/yarn-autocompletions ]
then
    mkdir $1/plugins/yarn-autocompletions
fi

cp ./yarn-autocompletions.plugin.zsh $1/plugins/yarn-autocompletions
cp ./yarn-deps $1/plugins/yarn-autocompletions
cp ./yarn-scripts $1/plugins/yarn-autocompletions
