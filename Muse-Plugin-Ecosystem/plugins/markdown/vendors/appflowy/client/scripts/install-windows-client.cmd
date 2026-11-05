@echo off
setlocal
where python >nul 2>&1 && python "%~dp0install-windows-client.py" %* && exit /b %ERRORLEVEL%
py -3 "%~dp0install-windows-client.py" %*
