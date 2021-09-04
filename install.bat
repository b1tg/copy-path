reg add HKEY_CLASSES_ROOT\*\shell\copypath\command /ve /t REG_SZ /d "D:\Downloads\copy-path-main\copy-path-main\x64\Debug\copy_path.exe %%1" /f


echo "[*] enter any key to exit"
pause