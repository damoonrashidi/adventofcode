read -r year day <<< "$@"
session=$(<.session)

mkdir "src/inputs/$year"

echo "Getting input for $year/$day"
curl "https://adventofcode.com/$year/day/$day/input" \
  -H 'authority: adventofcode.com' \
  -H 'accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7' \
  -H 'accept-language: en-US,en;q=0.9,sv;q=0.8' \
  -H 'cache-control: max-age=0' \
  -H "cookie: session=$session" \
  --compressed \
  -o "./src/inputs/$year/$day.txt";

echo "Creating template for $year/$day"

template=$(<src/template.txt)

updated_content="${template//\{year\}/$year}"
updated_content="${updated_content//\{day\}/$day}"

echo "$updated_content" > "./src/bin/${year}_${day}.rs"
