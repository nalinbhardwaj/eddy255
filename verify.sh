#!/bin/sh
sage safecurveverify.py SafeCurves

echo ""
echo "Eddy255 (SafeCurves)"
echo "---------------"
grep -Rn '.' SafeCurves/verify-* |grep '^SafeCurves/verify-.*:1:' |sed 's/:1:/ = /'
