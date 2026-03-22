# tauri + Svelte
electron에 비해 용량에 대한 이점 (15~20mb)

## rust 설치
[rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
`rustc --version`으로 확인

vscode에서 rust-analyzer 설치
`npm install -g @tauri-apps/cli` 설치

## 실행 (Debug)
`npm run tauri dev`

## 빌드 (exe 생성)
`npm run tauri build`
1. src-tauri/target/release/bundle/app.exe
2. src-tauri/target/nsis/installer.nsi

## Todo
- PCAN-basic.dll 호출, init, write, read 테스트