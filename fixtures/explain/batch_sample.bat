@echo off
setlocal enabledelayedexpansion

set count=0
for %%f in (*.txt) do (
    set /a count+=1
    echo File !count!: %%f
)

echo Total text files: !count!
endlocal
