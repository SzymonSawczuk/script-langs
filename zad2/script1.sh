#!/bin/bash

CONTINENT=""
YEAR=""
MONTH=""

for flag in $@
do
    case $flag in
        -c=* | --continent=*) CONTINENT=`echo $flag | sed -e 's/^[^=]*=//g'` ;;
        -y=* | --year=*) YEAR=`echo $flag | sed -e 's/^[^=]*=//g'` ;;
        -m=* | --month=*) MONTH=`echo $flag | sed -e 's/^[^=]*=//g'` ;;
        *) echo "Wrong option"
     esac

done

if [ ${#MONTH} == 1 ]
then
    MONTH="0$MONTH"
fi

if [ $MONTH -lt "1" ]
then
    echo "Wrong month number"
    exit 1
fi

if [ $MONTH -gt "12" ]
then
    echo "Wrong month number"
    exit 1
fi

echo "Number of cases for $CONTINENT *.$MONTH.$YEAR:"
echo `./process --select $CONTINENT .$MONTH.$YEAR --project 5 < covid.tsv | ./aggregate -s -n`
