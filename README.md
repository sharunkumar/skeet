# Skeet

Skeet from your command line for [Bluesky](https://bsky.app/) or other supported atproto sites

## Installation
```shell
cargo install skeet
```

## Environment Setup
Add the following environment variables:
```shell
SKEET_HOST - optional, defaults to https://bsky.social
SKEET_USERNAME - your username / handle
SKEET_PASSWORD - app password. you can create one at https://bsky.app/settings/app-passwords
```

## Usage
```shell
skeet <your text here>
```

You can also pipe some text from another command
```shell
echo hello | skeet
```

Pro tip: if you have [gum](https://github.com/charmbracelet/gum) installed, you can use the following command to have a multi-line editor before posting:
```shell
gum write | skeet
```
