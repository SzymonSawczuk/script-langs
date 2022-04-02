#!/bin/bash

INPUT="."
OUTPUT=""

for flag in $@
do
    case $flag in
        --src=*) INPUT=`echo $flag | sed -e 's/^[^=]*=//g'` ;;
        --dst=*) OUTPUT=`echo $flag | sed -e 's/^[^=]*=//g'` ;;
        *) echo "Wrong option"
     esac

done

if [ $OUTPUT == "" ]
then
    echo "Output directory name is an empty string"
    exit 2
fi

if [ ! -d $INPUT ]
then
    echo "Directory does not exist!"
    exit 1
fi

cp -R $INPUT $OUTPUT
