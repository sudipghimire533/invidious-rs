#!/usr/bin/bash

# assign $INVIDIOUS_DOMAIN env variable to INVIDIOUS_DOMAIN bash variable
# if that env is not set or is empty defaults to "https://vid.puffyan.us/api/v1/"
INVIDIOUS_DOMAIN=${INVIDIOUS_DOMAIN:-https://vid.puffyan.us/api/v1}

echo "Using instance $INVIDIOUS_DOMAIN .."

# takes onestring argument as url
# takes another string argument as file path
# make curl GET call to the url
# if response if ok save the curl response to file and return 0
# if not,return 1
function save_response() {
  curl -s "$1" >"$2";
  if [ "$?" -ne 0 ]; then
    echo "Call to $1 Failed.."
    return 1;
  else
    echo "FIle $2 updated with response from $1"
    return 0
  fi
}

save_response "$INVIDIOUS_DOMAIN/stats" "instance-stats.json"

save_response "$INVIDIOUS_DOMAIN/search?q=bartika-eam-rai" "search-all.json"

save_response "$INVIDIOUS_DOMAIN/channels/UCX6OQ3DkcsbYNE6H8uQQuVA" "channel-info.json"

save_response "$INVIDIOUS_DOMAIN/videos/Wx92JT6IrKY" "video-info.json"
