#!/usr/bin/env bash
set -euo pipefail

NAME="$1"
SRC="$2"

pwd=$(pwd)
echo "PWD=${pwd}"
root=$(git rev-parse --show-toplevel)

dst="${root}/references/assets/${NAME}"
mkdir -p "${dst}"

if [ -d "${SRC}" ]; then
	cp -r "${SRC}/." "${dst}/"
else
	cp "${SRC}" "${dst}/"
fi

cat >"${dst}/CREDITS.md" <<EOF
- [${NAME}](URL): DESCRIPTION</br>
  by [AUTHOR](AUTHOR_URL)
EOF

awk -v NAME="${NAME}" '
/^# TODO$/ {
    print "# TODO"
    print "- [" NAME "](URL): DESCRIPTION</br>"
    print "  by [AUTHOR](AUTHOR_URL)"
    print ""
}
1' "${root}/CREDITS.md" >"${root}/CREDITS.md.tmp" &&
	mv "${root}/CREDITS.md.tmp" "${root}/CREDITS.md"
