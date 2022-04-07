#!/bin/bash

AMOUNT="5"
COUNTRY=""

for flag in $@
do
    case $flag in
        -c=* | --country=*) COUNTRY=`echo $flag | sed -e 's/^[^=]*=//g'` ;;
        -a=* | --amount=*) AMOUNT=`echo $flag | sed -e 's/^[^=]*=//g'` ;;
        *) echo "Wrong option"
     esac

done

countries=()

if [[ $COUNTRY == "" ]]
then
    countries=(`./process --project 7 < covid.tsv | uniq`)
    countries=${countries[@]:1}
else
    IFS=','
    read -r -a countries <<< "$COUNTRY"
fi    

if (( $(echo "$AMOUNT > ${#countries[@]}" | bc -l) ))
then
    AMOUNT=${#countries[@]}
fi

deaths=0.0
cases=0.0

echo "$AMOUNT best countries compared by deaths/cases percent:"

for country in ${countries[@]}
do
    deaths_per_cases=0
    deaths=(`./process --select $country --project 6  < covid.tsv | ./aggregate -s -n `)
    cases=(`./process --select $country --project 5  < covid.tsv | ./aggregate -s -n `)
    if (( $(echo "$cases.0>0" | bc -l) ))
    then 
        deaths_per_cases=`echo "$deaths/$cases * 100" | bc -l`
    fi    
    echo -e "$country\t$deaths_per_cases"
done | LC_ALL=C sort -gk 2 | ./head --lines $AMOUNT -e
