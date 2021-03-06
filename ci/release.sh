#!/bin/bash -lx

if ! jq --version >/dev/null 2>&1 ; then
    echo "No jq in \$PATH" 1>&2
    exit 1
fi

if [ -z $OAUTH ]; then
    echo "No \$OAUTH in environment" 1>&2
    exit 1
fi

if [ $# -ne 1 ]; then
    echo "Usage $(basename $0) /organisation/repo" 1>&2
    exit 1
fi

# get version from Cargo.toml
version=$(grep ^version <Cargo.toml | cut -d\" -f2 2>/dev/null)

if [ -z $version ]; then
    echo "Couldn't get version from Cargo.toml"
    exit 1
fi

release=https://api.github.com/repos/$1/releases

# get release id
release_id=$(
    curl -s $release \
        | jq -r '.[] | "\(.tag_name) \(.id)"' \
        | grep -F $version | head -n1 | cut -d' ' -f2
)

# create a release if none already for this version
if [ -z $release_id ]; then
    echo "Creating release for $version"

    release_id=$(curl -s -X POST -d '{"tag_name":"'$version'"}' $release?access_token=$OAUTH | jq -r '"\(.id)"' 2>/dev/null)

    echo "Release ID: $release_id"

    if [ "$release_id" = "null" ] || [ -z $release_id ]; then
        echo "Failed to find/create release" 1>&2
        exit 1
    fi
else
    echo "Updating assets for release $version"
fi

# get upload url
upload=$(curl -s $release/$release_id?access_token=$OAUTH | jq -r '"\(.upload_url)"' | cut -d'{' -f1)
for i in assets/*.tar.gz; do
    echo "Asset: $i"

    if [ -f $i ]; then
        asset_name=$(basename $i)
        echo "Asset name: $asset_name"
        asset_id=$(
            curl -s $release/$release_id/assets?access_token=$OAUTH \
                | jq -r '.[] | "\(.name) \(.id)"' \
                | grep -F $asset_name | cut -d' ' -f2
                )
        # delete asset if already exists
        if ! [ -z $asset_id ]; then
            echo "Deleting existing $asset_name asset"
            curl -s -X DELETE $release/assets/$asset_id?access_token=$OAUTH
        fi
        # upload asset
        echo "Uploading $asset_name asset"
        curl -X POST -H "Content-Type: application/tar+gzip" --data-binary @"$i" "$upload?name=$asset_name&access_token=$OAUTH" >/dev/null
    fi
done
