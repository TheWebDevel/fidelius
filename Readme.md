# Fidelius ✨
A TOTP generator for 2FA. Works with any service that can offer you a secret key for 2FA.

✅ OTP Copied to Clipboard!

## Installation

### Mac OS x86_64-apple-darwin

```
curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- --git thewebdevel/fidelius --target x86_64-apple-darwin
```

## Usage

### Setup
1. Get secret key from apps like `Github or Twitter` to enable 2FA.
2. Set Key using `fidelius <SERVICE> --set-key <KEY>`. Example: `fidelius twitter --set-key abcdefgh`.
3. `OTP` will be copied to your clipboard. Use that to complete enabling 2FA.

#### Usage
`fidelius twitter --generate` to copy the OTP to Clipboard.

---

### Export
You can export your keys to a location you need using `fidelius export --path <PATH>`.

Example: `fidelius export --path /Users/sathish/Desktop/2fa-keys.json`


### Import
Upgrading your computer? Do you use two computers? No Worries! You can import the `json` file and start using it straight away. It's that easy using `fidelius import --path <PATH>`.

Example: `fidelius import --path /Users/sathish/Desktop/2fa-keys.json`


### Fidelius Charm in Harry Potter
```
    "An immensely complex spell involving the magical concealment of a secret inside a single, living soul. The information is hidden inside the chosen person, or Secret-Keeper, and is henceforth impossible to find — unless, of course, the Secret-Keeper chooses to divulge it. As long as the Secret-Keeper refused to speak, You-Know-Who could search the village where Lily and James were staying for years and never find them, not even if he had his nose pressed against their sitting room window!"
    —Filius Flitwick's description of the charm
```

Here, your secret keeper is your computer.