#!/bin/sh
decrypt_link() {
    secret_key='3633393736383832383733353539383139363339393838303830383230393037'
    iv='34373730343738393639343138323637'
    ajax_url="https://gogoplay4.com/encrypt-ajax.php"
    id=$(printf "%s" "$1" | sed -nE 's/.*id=(.*)&title.*/\1/p')
    
    # printf 'id: %s\n' "$id"

    ajax=$(echo $id|openssl enc -e -aes256 -K "$secret_key" -iv "$iv" | base64)

    # printf 'ajax: %s\n' "$ajax"

    data=$(curl -s -H "X-Requested-With:XMLHttpRequest" "$ajax_url" -d "id=$ajax" | sed -e 's/{"data":"//' -e 's/"}/\n/' -e 's/\\//g')
    
    # printf 'data: %s\n' "$data"
    
    printf '%s' "$data" | base64 -d | openssl enc -d -aes256 -K "$secret_key" -iv "$iv" | sed -e 's/\].*/\]/' -e 's/\\//g' |
        grep -Eo 'https:\/\/[-a-zA-Z0-9@:%._\+~#=][a-zA-Z0-9][-a-zA-Z0-9@:%_\+.~#?&\/\/=]*'
}

decrypt_link $1