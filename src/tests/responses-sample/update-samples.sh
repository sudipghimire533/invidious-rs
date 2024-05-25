#!/usr/bin/env bash

# assign $INVIDIOUS_DOMAIN env variable to INVIDIOUS_DOMAIN bash variable
# if that env is not set or is empty defaults to "https://vid.puffyan.us/api/v1/"
INVIDIOUS_DOMAIN=${INVIDIOUS_DOMAIN:-https://vid.puffyan.us/api/v1}

echo "Using instance $INVIDIOUS_DOMAIN .."

# takes onestring argument as url
# takes another string argument as file path
# make curl GET call to the url
# if response if ok save the curl response to file and return 0
# if not,return 1

DEST_DIR=$(dirname "$0")
function save_response() {
  URL=$2
  FILE_NAME=$1
  DEST_FILE=$DEST_DIR/$FILE_NAME

  curl -s "$URL" >"$DEST_FILE";
  if [ "$?" -ne 0 ]; then
    echo "Call to $URL Failed.."
    return 1;
  else
    echo "FIle $FILE_NAME updated with response from $URL"
    return 0
  fi
}

save_response "instance-stats.json"   "$INVIDIOUS_DOMAIN/stats" 

save_response "search-all.json"       "$INVIDIOUS_DOMAIN/search?q=bartika-eam-rai" 

save_response "channel-info.json"     "$INVIDIOUS_DOMAIN/channels/UCX6OQ3DkcsbYNE6H8uQQuVA"

save_response "video-info.json"       "$INVIDIOUS_DOMAIN/videos/Wx92JT6IrKY"

save_response "playlist-info.json"    "$INVIDIOUS_DOMAIN/playlist/PLm3v_EFg1EILrg_6fHX6kt2iEoIbFu6n4"

