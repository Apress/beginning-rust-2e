@echo off
if not exist target mkdir target
if exist %1.rs (
    echo File %1.rs
    type %1.rs
    echo ____________ ____________ ____________ ____________
    rustc --edition 2021 -O -o target\%1.exe %1.rs
) else if exist %1.c (
    echo File %1.c
    type %1.c
    echo ____________ ____________ ____________ ____________
    cl /nologo /Ox /Fetarget\%1.exe %1.c
) else if exist %1.cpp (
    echo File %1.cpp
    type %1.cpp
    echo ____________ ____________ ____________ ____________
    cl /nologo /Ox /EHsc /Fetarget\%1.exe %1.cpp
) else (
    echo Invalid file name: %1
    exit /B
)
if errorlevel 1 goto end
target\%1
:end
