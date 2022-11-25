# ![qw-scripts Banner](https://i.imgur.com/68jLFg3.png)

## qw-scripts discord

<https://dsc.gg/qw-scripts>

## Intro

you can use this command line application to easily create new FiveM resources. It is currently setup to just create resources for QBCore but will evetually be setup to be framework agnostic as I work on it more down the road.

## Command Line Interface

I would recommend installing this somewhere and adding it to your path so you can use it anywhere. The following is the current use of the CLI tool.

```
fivemres.exe -p <project_name>
```

This will create all the folders and files for you and insert some basic information like creating a local variable in client and server to use the QBCore core object, it will create a barebones `fxmanifest.lua` for you and insert some basic info into that as well.
