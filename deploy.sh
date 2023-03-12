#!/bin/sh

trunk build --release --public-url foxhole/ && cp ./dist/* ./docs/
