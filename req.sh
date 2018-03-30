#!/bin/sh

API_KEY=$(cat key)
TIMESTAMP=$(date "+%s")

#echo $TIMESTAMP
#echo $API_KEY
curl "https://maps.googleapis.com/maps/api/distancematrix/json?units=imperial&origins=Sandgasse%2024%20Graz&destinations=St.%20Nikolai%20i.S.&departure_time=$TIMESTAMP&traffic_model=pessimistic&key=$API_KEY"
