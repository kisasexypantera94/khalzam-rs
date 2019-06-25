# Khalzam
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/cacf5d6c8e6743fab59209e24f58ca4f)](https://app.codacy.com/app/kisasexypantera94/khalzam-rs?utm_source=github.com&utm_medium=referral&utm_content=kisasexypantera94/khalzam-rs&utm_campaign=Badge_Grade_Dashboard)
[![Latest Version](https://img.shields.io/crates/v/khalzam.svg)](https://crates.io/crates/khalzam)
[![Latest Version](https://docs.rs/khalzam/badge.svg)](https://docs.rs/khalzam)
## About
This is a port of my recent project [khalzam-go](https://github.com/kisasexypantera94/khalzam).

`khalzam` is an audio recognition library that makes it easy to index and recognize audio files.
It focuses on speed, efficiency and simplicity.
Its algrorithm is based on [this article](https://royvanrijn.com/blog/2010/06/creating-shazam-in-java/).

## Setup
You need to create and initialize database:
```zsh
cd src/db
createdb -O user databasename
psql -f initdb.sql databasename
```

## Usage
You can use the library either
through the [API](https://github.com/kisasexypantera94/khalzam-rs/tree/master/examples)
or using [khalzam-cli](https://github.com/kisasexypantera94/khalzam-cli)
