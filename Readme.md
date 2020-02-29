# Fidelius
A TOTP generator for 2FA Authentication.

## Usage

### Setup
1. Get secret key from apps like `Github or Twitter`.
2. Set Key using `fidelius <SERVICE> --set-key <KEY>`. Example: `fidelius twitter --set-key abcdefgh`.
3. A `OTP` will be copied to your clipboard. Use that to complete enabling the 2FA Authentication.

#### Usage
`fidelius twitter --generate`