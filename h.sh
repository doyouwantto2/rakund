DIR="src"

if [ ! -d "$DIR" ]; then
  echo "⚠️  Directory '$DIR' not found."
  exit 1
fi

find "$DIR" -type f | sort | while read -r FILE; do
  echo ""
  echo "════════════════════════════════════════════════════════════════"
  echo "📄 $FILE"
  echo "════════════════════════════════════════════════════════════════"
  echo ""
  cat "$FILE"
  echo ""
done

echo ""
echo "✅ Done."
