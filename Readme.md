# Fidelius ✨
A TOTP generator for 2FA Authentication.

## Usage

### Setup
1. Get secret key from apps like `Github or Twitter` to enable 2FA.
2. Set Key using `fidelius <SERVICE> --set-key <KEY>`. Example: `fidelius twitter --set-key abcdefgh`.
3. `OTP` will be copied to your clipboard. Use that to complete enabling 2FA.

#### Usage
`fidelius twitter --generate` to copy the OTP to Clipboard.