echo "Please build editor first!"

@REM UNSOLVED: pnpm causes exit
@REM pushd editor
@REM pnpm run build
@REM popd ..

uv run pyinstaller core\__main__.py --onedir --noconfirm --noconsole

rmdir /S /Q src-tauri\resources

xcopy /E /I /Y editor\dist tauri\src-tauri\resources\editor\dist
xcopy /E /I /Y dist\__main__ tauri\src-tauri\resources

cd tauri

npx tauri build