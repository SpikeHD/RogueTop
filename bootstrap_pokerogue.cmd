@REM This script requires Git, pnpm, and Node.js to be installed.
git clone git@github.com:pagefaultgames/pokerogue.git src-ext --depth 1

cd src-ext

pnpm install
pnpm build