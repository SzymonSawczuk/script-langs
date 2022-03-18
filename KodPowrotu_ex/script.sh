#!/bin/bash

read -p "Podaj parametry rodzielone spacja: " -a args
read -p "Podaj tekst albo nazwe pliku z tekstem: " path_to_file

if [[ -f $path_to_file ]] 
then
    ./KodPowrotu ${args[@]} < $path_to_file
else
    echo -e "Wpisano tekst!\n"
    echo $path_to_file | ./KodPowrotu ${args[@]}
fi


status=$?

if [[ $status -gt 0 ]]
then
    echo -e "Najwiecej wystapien: "${args[$status - 1]}
else 
    echo -e "Zaden parametr nie wystapil ani razu!" 
fi
