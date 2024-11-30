#!/bin/bash

printf '\033[1m ========== MAIDONO : PACK ==========\033[0m\n'

while test $# -gt 0
do
    case "$1" in
        --skip-build) MAIDONO__SKIP_BUILD=1
            ;;
        --*) printf " \033[93mWarning: unknown option '%s'\033[0m\n" "$1"
            ;;
        *) printf " \033[93mWarning: got unexpected arguments\033[0m\n"
            ;;
    esac
    shift
done

MAIDONO__enter_project_dir() {
  cd "$1" || {
    printf '\033[91m Couldn''t open project dir '%s'.\n Make sure this script is executed at the root of the git repository.\033[0m\n' "$1";
    exit 1;
  }
}

RUST_TARGET=$( rustc -vV | sed -n 's|host: ||p' )
printf ' Building executables for target [%s]' "$RUST_TARGET"

if [[ -z "$MAIDONO__SKIP_BUILD" ]]; then
  # 1. maidctl build

  MAIDONO__enter_project_dir maidctl
  printf "\n\033[1;94m Building maidctl...\033[0m\n\n"
  cargo build -r
  cd ..

  # 2. maidono build

  MAIDONO__enter_project_dir maidono
  printf "\n\033[1;94m Building maidono...\033[0m\n\n"
  cargo build -r
  cd ..

  # 3. web app build
  MAIDONO__enter_project_dir web
  printf "\n\033[1;94m Building web app...\033[0m\n\n"
  npm run build
  cd ..
else
  printf "\n\033[90m Builds skipped\033[0m\n"
fi

# 4. create .archive dir
printf "\n\033[1;94m Packing everything...\033[0m\n"
if [[ -d .archive ]]; then
  printf "\033[90m   temp dir already exists, cleaning up..."
  rm -rf .archive/*
  printf " done\033[0m\n"
else
  mkdir .archive
fi

# 4. copy files
MAIDONO_copy_file_to_archive() {
  printf "   %s -> %s\n" "$1" "$2"
  mkdir -p ".archive/$(dirname "$2")"
  if [[ "$3" == "recursive" ]]; then
    cp -r "$1" ".archive/$2"
  else
    cp "$1" ".archive/$2"
  fi
}
MAIDONO_copy_dir_to_archive() {
  printf "   %s/* -> %s\n" "$1" "$2"
  mkdir -p ".archive/$(dirname "$2")"
  cp -r "$1" ".archive/$2"
}

MAIDONO_copy_file_to_archive maidctl/target/release/maidctl bin/maidctl
MAIDONO_copy_file_to_archive maidono/target/release/maidono bin/maidono
MAIDONO_copy_file_to_archive scripts/install.sh install.sh
MAIDONO_copy_dir_to_archive web/dist web
MAIDONO_copy_file_to_archive README.md README.md
tar -czf "maidono-0.1.0-$RUST_TARGET.tar.gz" -C .archive .
rm -r .archive

exit 0
