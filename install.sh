if [ ! -d $1/yarn-autocompletions ]
then
    mkdir $1/yarn-autocompletions
fi

cp ./yarn-autocompletions.plugin.zsh $1/yarn-autocompletions
cp ./yarn-autocompletions $1/yarn-autocompletions
