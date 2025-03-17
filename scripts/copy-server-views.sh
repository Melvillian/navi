#!/usr/bin/env bash

viewsPath="server/views"
test -d ${viewsPath} && cp -r ./${viewsPath} ./dist/${viewsPath}
