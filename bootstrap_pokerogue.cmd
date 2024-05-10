@REM This script requires Git, pnpm, 7zip, and Node.js to be installed.
git clone https://github.com/pagefaultgames/pokerogue.git src-ext --depth 1

cd src-ext

pnpm install

@REM Write "VITE_BYPASS_LOGIN" to .env file
echo VITE_BYPASS_LOGIN="1" > .env

pnpm build

@REM Compress dist folder to "game.dat"
cd dist
7z a -tzip -mx5 -r ../../game.dat *

cd ../..
