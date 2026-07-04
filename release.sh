#!/bin/bash
CURRENT_VERSION=$(svu current)
NEW_VERSION=$(svu next)

if [ "$CURRENT_VERSION" != "$NEW_VERSION" ] ; then
    cargo set-version "${NEW_VERSION:1}"
    git add Cargo.toml
    git commit -m "chore(release): prepare for $NEW_VERSION"
    git tag $NEW_VERSION

    if git-cliff --current -o CHANGELOG.md ; then

        echo "Building version: $NEW_VERSION"
        export CARGO_TARGET_DIR=target/cross
        if cargo build --target --release ; then
            echo "Done!"
            git push origin HEAD
            git push origin $NEW_VERSION

            echo "Uploading to GitHub..."
            gh release create $NEW_VERSION target/release/rustcdn --notes-file CHANGELOG.md
            echo "Done!"
        else
            echo "Unable to build, cleaning up..."
            git tag -d $NEW_VERSION
            git reset --hard HEAD~1 # HMMMMMMMMMMM
        fi

    else
        git tag -d $NEW_VERSION
        git reset --hard HEAD~1 # HMMMMMMMMMMM
        echo "Unable to generate changelog, aborted"
    fi

else 
    echo "No new commits since last version, aborted"
fi
