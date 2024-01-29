@echo off
goto :start
GB 2024/01/22
controlla se l'ultimo file di backup e' piu' vecchio di un mese e in caso crea una copia
Lanciato da directory in cui c'e' il file da copiare crea copia in directory specificata con <nome>_<anno>_<mese>_<giorno>.<ext>

check if the last backup file is older than a month
in this case make a copy in a specified directory

:start
echo Batch per il backup del file in .\backupDir 
echo.
pause

set workingDirectory=".\backupDir"
set namefileToBackup="filename"
set extfileToBackup=".extension"

for /f "tokens=1,2,3,6 eol= skip=5 delims=;/ " %%G in ('dir /o:-d /t:c /a:-d %workingDirectory:"=%\*.%extfileToBackup:"=%') do (call :forBody %%G %%H %%I "%%J" & goto :myeof)

:myeof
pause
goto :eof
:::::::::::::::::::::::::::::
:forBody
::::controllo nome file
::::check filename
if [%4] == [] goto :eof
set filename=%4
set relativePath="%workingDirectory:"=%\%filename:"=%" 
if not exist %relativePath% ( echo relativePath does not exist & goto :eof)

::get data odierna
::get today
set year=%date:~6,4%
set month=%date:~3,2%
set day=%date:~0,2%
set completeDate=%year%%month%%day%

::calcolo ultima data ammessa, ovvero un mese prima
::calculate date - (1 month)
if %month% NEQ 1 ( 
    set /a "monthBefore=%month% - 1"
    set /a yearBefore=%year%
) else (
    set /a yearBefore=%year%-1
    set /a monthBefore=12
)
set "monthBefore=0%monthBefore%"
set oneMonthBefore=%yearBefore%%monthBefore:~-2%%day%

::get data file
if [%1] == [] goto :eof
if [%2] == [] goto :eof
if [%3] == [] goto :eof
set yearF=%3
set monthF=%2
set dayF=%1
set completeDateF=%yearF%%monthF%%dayF%

::echo completeDate: [%completeDate%] & echo oneMonthBefore: [%oneMonthBefore%] & echo completeDateF: [%completeDateF%]

::verifica data
::check date
if %completeDateF% GTR %completeDate% (
    echo Errore data
    exit /b
) 

::confronto date 
if %completeDateF% LEQ %oneMonthBefore% (
    call :copiaFile
) else (
    echo ---------------------------------------------
    echo Il file e' gia' recente
    echo ---------------------------------------------
)

exit /B
:::::::::::::::::
:copiaFile
set src=".\%namefileToBackup:"=%%extfileToBackup:"=%"
set dest="%workingDirectory:"=%\%namefileToBackup:"=%_%year%_%month%_%day%%extfileToBackup:"=%"
echo ---------------------------------------------
echo copiando %src% in %dest%
echo ---------------------------------------------
copy %src% %dest%
exit /B




