#!/bin/bash 
MOUNT_PATH=$1
GO_TERM=$2
JSON_NAME=$3
curl "https://api.geneontology.org/api/bioentity/function/GO:${GO_TERM}?category=gene&taxon_label=Homo%20sapiens&rows=1000000" -H "accept: application/json" > ${MOUNT_PATH}/${JSON_NAME}.json
