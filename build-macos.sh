#!/bin/zsh
mkdir -p macos-pkg

if [ ! -f "macos-pkg/R-4.5.1-arm64.pkg" ]; then
  curl -L -o macos-pkg/R-4.5.1-arm64.pkg https://cran.r-project.org/bin/macosx/big-sur-arm64/base/R-4.5.1-arm64.pkg
fi

if [ ! -f "macos-pkg/rv.tar.gz" ]; then
  curl -L -o macos-pkg/rv.tar.gz https://github.com/A2-ai/rv/releases/download/v0.13.2/rv-v0.13.2-aarch64-apple-darwin.tar.gz
fi

if [ ! -f "macos-pkg/rv" ]; then
  tar -xzf macos-pkg/rv.tar.gz -C macos-pkg/
fi

rm -rf ./macos-pkg/tmp-r-pkg
pkgutil --expand-full ./macos-pkg/R-4.5.1-arm64.pkg ./macos-pkg/tmp-r-pkg
rm -rf src-tauri/local-r
cp -Lr ./macos-pkg/tmp-r-pkg/R-fw.pkg/Payload/R.framework/Resources src-tauri/local-r

sed -i '' '/if test -n "\${R_HOME}" && \\/,/export R_DOC_DIR/c\
if test -z "${R_HOME}"; then\
  R_HOME="${R_HOME_DIR}"\
fi\
export R_HOME\
\
if test -z "${R_SHARE_DIR}"; then\
  R_SHARE_DIR="${R_HOME}/share"\
fi\
export R_SHARE_DIR\
\
if test -z "${R_INCLUDE_DIR}"; then\
  R_INCLUDE_DIR="${R_HOME}/include"\
fi\
export R_INCLUDE_DIR\
\
if test -z "${R_DOC_DIR}"; then\
  R_DOC_DIR="${R_HOME}/doc"\
fi\
export R_DOC_DIR
' src-tauri/local-r/R

cp -L macos-pkg/rv src-tauri/local-r

if [ ! -d "shiny-app/" ]; then
  echo "Please put the contents of your rv project with a Shiny App in shiny-app/"
  exit 1
fi

rm -rf src-tauri/app
cp -Lr shiny-app src-tauri/app
find src-tauri/app -empty -print | xargs rm

# Run rv sync using the R version that is installed in local-r
# use the same logic as the lib.rs
LOCAL_R_PATH="$(pwd)/src-tauri/local-r"
LOCAL_R_LIB_PATH="${LOCAL_R_PATH}/lib"

export R_HOME="${LOCAL_R_PATH}"
export PATH="${LOCAL_R_PATH}${PATH:+:$PATH}"
export LD_LIBRARY_PATH="${LOCAL_R_LIB_PATH}${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export DYLD_LIBRARY_PATH="${LOCAL_R_LIB_PATH}${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"

(cd src-tauri/app && rv sync)

cargo tauri build --bundles dmg
