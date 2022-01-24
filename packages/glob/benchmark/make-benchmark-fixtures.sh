# Copied from https://github.com/isaacs/node-glob

# The ISC License

# Copyright (c) Isaac Z. Schlueter and Contributors

# Permission to use, copy, modify, and/or distribute this software for any
# purpose with or without fee is hereby granted, provided that the above
# copyright notice and this permission notice appear in all copies.

# THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
# WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
# MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
# ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
# WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
# ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR
# IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

# ## Glob Logo

# Glob's logo created by Tanya Brassie <http://tanyabrassie.com/>, licensed
# under a Creative Commons Attribution-ShareAlike 4.0 International License
# https://creativecommons.org/licenses/by-sa/4.0/

#!/bin/bash

tmp=${TMPDIR:-/tmp}
export CDPATH=
set -e
rm -rf $tmp/benchmark-fixture
mkdir -p $tmp/benchmark-fixture
if ! [ -d $tmp/benchmark-fixture ]; then
  echo Making benchmark fixtures
  mkdir $tmp/benchmark-fixture
  cd $tmp/benchmark-fixture
  dirnames=`echo {0..9}/{0..9}/{0..9}/{0..9}` # 10000 dirs
  filenames=`echo {0..9}/{0..9}/{0..9}/{0..9}/{0..9}.txt`
  echo $dirnames | xargs mkdir -p
  echo $filenames | xargs touch
fi