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


if [ $AMOUNT -lt "0" ]
then
    echo "Wrong amount number"
    exit 1
fi

min_year=`./process --select $COUNTRY --project 4 < covid.tsv | ./aggregate --minimum -n`
max_year=`./process --select $COUNTRY --project 4 < covid.tsv | ./aggregate --maximum -n`

years=( $(seq $min_year $max_year) )
months=("01" "02" "03" "04" "05" "06" "07" "08" "09" "10" "11" "12")
deaths=()

for year in ${years[@]}
do 
    for month in ${months[@]}
    do
       
        deaths+=(`./process --select $COUNTRY .$month.$year --project 6 < covid.tsv | ./aggregate -s -n`)
        
    done
done

if (( $(echo "$AMOUNT > ${#deaths[@]}" | bc -l) ))
then
    AMOUNT=${#deaths[@]}
fi

index=0

echo "$AMOUNT best months for $COUNTRY:"

for year in ${years[@]}
do 
    for month in ${months[@]}
    do
        echo -e "*.$month.$year\t ${deaths[index]}" 
        index=$(($index+1))
    done
done | sort -nk 2 | ./head --lines $AMOUNT -e

echo "$AMOUNT worst months for $COUNTRY:"

index=0
for year in ${years[@]}
do 
    for month in ${months[@]}
    do
        echo -e "*.$month.$year\t ${deaths[index]}" 
        index=$(($index+1))
    done
done | sort -nrk 2 | ./head --lines $AMOUNT -e
