<img src=".github/notsopkg.png" alt="Logo" align="right" width="150"/>

<div id="user-content-toc">
  <ul style="list-style: none;">
    <summary>
      <h1>NotSoPkg</h1>
    </summary>
</ul>

Extract macOS XAR `.pkg`s on any platform

## Usage
`notsopkg <INPUT> [OUTPUT] --overwrite`

Output is optional, specify --overwrite if you want to overwrite existing extracted contents (if any)

## Developing
```
git clone https://github.com/oomfinator/NotSoPkg
cd NotSoPkg
cargo run -- "/path/to/.pkg" --overwrite
```

## Licence
Licensed under Apache v2