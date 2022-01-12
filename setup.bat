@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat"

REM used indirectly by calling with setup.bat as argument like 'setup.bat [build|...|clean]'
if "%~1" == "" echo "Please enter a valid argument"
if "%~1" == "check" goto check
if "%~1" == "build" goto build
if "%~1" == "run" goto run
if "%~1" == "doc" goto doc
if "%~1" == "clean" goto clean
if "%~1" == "help" goto help

REM arguments used if setup.bat is called with no arguments or run manually
:arguments
set /p arg= "Usage: [check|build|run|doc|clean|help|end]: "
if "%arg%" == "check" goto check
if "%arg%" == "build" goto build
if "%arg%" == "run" goto run
if "%arg%" == "doc" goto doc
if "%arg%" == "clean" goto clean
if "%arg%" == "help" goto help
if "%arg%" == "end" goto end

REM Want help?
:help
goto arguments

:end
exit

:check
cargo check
pause
goto end

:build
cargo build
pause
goto end

:run
cargo run
pause
goto end

:doc
cargo doc
pause
goto end

:clean
cargo clean
pause
goto end
