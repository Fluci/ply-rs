#!/bin/sh

# License: CC0 1.0 Universal
# https://creativecommons.org/publicdomain/zero/1.0/legalcode

set -ex

echo "# Reading config"
. ./scripts/travis-doc-upload.cfg

# filter such that only one job commits
#[ "$TRAVIS_OS_NAME" = "linux" ]
[ "${TRAVIS_BUILD_NUMBER}.1" = "${TRAVIS_JOB_NUMBER}" ]

[ "$TRAVIS_BRANCH" = master ] || [ "$TRAVIS_BRANCH" = travis_docs ]

[ "$TRAVIS_PULL_REQUEST" = false ]

eval key=\$encrypted_${SSH_KEY_TRAVIS_ID}_key
eval iv=\$encrypted_${SSH_KEY_TRAVIS_ID}_iv

echo "# Setting up .ssh"
mkdir -p ~/.ssh
openssl aes-256-cbc -K $key -iv $iv -in scripts/id_rsa.enc -out ~/.ssh/id_rsa -d
chmod 600 ~/.ssh/id_rsa

echo "# Cloning repository ${DOCS_REPO}"
git clone --branch gh-pages git@github.com:$DOCS_REPO deploy_docs

echo "# Configuring git"
cd deploy_docs
git config user.name "doc upload bot"
git config user.email "nobody@example.com"

echo "# Deleting old documentation"
rm -rf $PROJECT_NAME

echo "# Moving new documentation into repository"
mv ../target/doc $PROJECT_NAME

echo "# Adding, committing and pushing to origin."
git add -A $PROJECT_NAME
git commit -qm "doc upload for $PROJECT_NAME ($TRAVIS_REPO_SLUG)"
git push -q origin gh-pages
