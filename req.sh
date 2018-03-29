#!/bin/sh

API_KEY=$(cat key)

curl https://maps.googleapis.com/maps/api/distancematrix/json?units=imperial&origins=Washington,DC&destinations=New+York+City,NY&key=$API_KEY
