#! /bin/env bash
set -e

BASE_DIR="src/examples/examples-data/"

result=$(redis-cli --raw LPOP pr-queue)

if [[ -z $result ]]; then
	echo "no data in redis"
	exit 1
fi

command_name=$(echo $result | jq -r '.command.name')
mkdir -p "${BASE_DIR}/${command_name}"

NEW_UUID=$(cat /dev/urandom | tr -dc 'a-z0-9' | fold -w 6 | head -n 1)
filename=$(\
echo $result | \
	jq -r '.command.description' | \
	sed 's/ /-/g' | \
	tr '[:upper:]' '[:lower:]' | \
	awk -v NEW_UUID=$NEW_UUID '{print substr($0, 0, 40) "-" NEW_UUID ".json" }')

git checkout -b ${filename}
new_file_path="${BASE_DIR}/${command_name}/$filename"

echo ${result} | jq .  > ${new_file_path}
git add ${new_file_path}
git commit -m "added new example $filename automatically" 
git push -u origin ${filename}
git checkout master
git pull origin master
git branch -D ${filename}
