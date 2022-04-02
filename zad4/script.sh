#!/bin/bash
IFS=$'\n' 

./Paths -R -s | ./process --select $1 --project 1 3 | sort -nr | ./head --lines $2 -e

  

lines=(`./Paths -R -s | ./process --select $1 --project 3`)
result=0


 for line in ${lines[@]}
 do

        # echo $line
        wc_result=`wc -w "$line" | awk '{ print $1 }'`
        # echo $wc_result
        result=$(($result + $wc_result))
    
done

echo "średnia długość plików o rozszerzeniu $1 wynosi $(($result / ${#lines[@]})) słów"
