# Fidelius ✨
A TOTP generator for 2FA Authentication.

## Usage

### Setup
1. Get secret key from apps like `Github or Twitter` to enable 2FA.
2. Set Key using `fidelius <SERVICE> --set-key <KEY>`. Example: `fidelius twitter --set-key abcdefgh`.
3. `OTP` will be copied to your clipboard. Use that to complete enabling 2FA.

#### Usage
`fidelius twitter --generate` to copy the OTP to Clipboard.

## LICENSE
The MIT License (MIT)

Copyright (c) 2020 Sathish Kumar

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.