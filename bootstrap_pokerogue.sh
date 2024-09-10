# This script requires Git, pnpm, and Node.js to be installed.
git clone https://github.com/pagefaultgames/pokerogue.git src-ext --depth 1

cd src-ext

pnpm install

# Write "VITE_BYPASS_LOGIN" to .env file
echo VITE_BYPASS_LOGIN="1" > .env

pnpm build

# Compress dist folder to "game.dat"
cd dist
zip -9 -q -r game.zip .
mv game.zip ../../game.dat

cd ../..
