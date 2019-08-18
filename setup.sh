#!/bin/bash

cd src/db
createdb khalzam
psql -f initdb.sql khalzam
