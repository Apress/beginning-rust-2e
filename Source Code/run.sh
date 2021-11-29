[ -d target ] || mkdir target
if [ -f $1.rs ]; then
    echo File $1.rs
    cat $1.rs
    echo ____________ ____________ ____________ ____________
    rustc --edition 2021 -O -o target/$1 $1.rs
elif [ -f $1.c ]; then
    echo File $1.c
    cat $1.c
    echo ____________ ____________ ____________ ____________
    gcc -O -o target/$1 $1.c
elif [ -f $1.cpp ]; then
    echo File $1.cpp
    cat $1.cpp
    echo ____________ ____________ ____________ ____________
    g++ -O -o target/$1 $1.cpp
else
    echo Inexistent program source: $1
    exit
fi
[ $? = 0 ] && target/$1
