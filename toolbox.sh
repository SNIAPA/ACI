if [ "$EUID" = 0 ]
  then echo "do not run as root"
  exit
fi
set -e

inject()
{
  proc=$(pidof linux_64_client)

  if [ -z "$proc" ] 
  then
    echo "AssaultCube is not running"
    exit
  fi

  lib=$(realpath ./target/x86_64-unknown-linux-gnu/debug/libaci.so)
  echo $lib

  sudo gdb -n -q -p $proc -batch \
    -ex "echo \033[1mCalling dlopen\033[0m\n" \
    -ex "call ((void*(*)(const char*, int))dlopen)(\"$lib\",3)" \
    -ex "echo \033[1mCalling dlerror\033[0m\n"                          \
    -ex "call ((char*(*)(void))dlerror)()"                             
}
help()
{
  echo -e "options:\n\tinject\n\thelp\n\tattach\n\tbuild\n\tbai - build and inject"
}
attach(){
  proc=$(pidof linux_64_client)

  if [ -z "$proc" ] 
  then
    echo "AssaultCube is not running"
    exit
  fi

  sudo gdb -q -p $proc
}
build(){
  cargo build
}
bai(){
  build
  inject
}


if [ -z "$1" ] 
then
  help
fi

case "$1" in
  help)
    help
    ;;
  inject)
    inject
    ;;
  attach)
    attach
    ;;
  build)
    build
    ;;
  bai)
    bai
    ;;
esac
