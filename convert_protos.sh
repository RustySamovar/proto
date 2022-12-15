#!/bin/bash

PROTOS=`find Sorapointa-Protos/proto/ -type f -name *.proto`

for f in ${PROTOS}; do
  x=`echo $f | xargs basename`;
  cat $f | sed 's/proto3";/\0\n\npackage Proto;/' > protobuf/$x;
  echo $x;
done
