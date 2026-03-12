#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/docs/nodit-official"
BASE_URL="https://developer.nodit.io"

mkdir -p "${OUT_DIR}/raw"

echo "Fetching discovery files..."
curl -fsSL "${BASE_URL}/llms.txt" -o "${OUT_DIR}/llms.txt"
curl -fsSL "${BASE_URL}/sitemap.xml" -o "${OUT_DIR}/sitemap.xml"

URL_LIST_FILE="${OUT_DIR}/urls.txt"
MANIFEST_FILE="${OUT_DIR}/manifest.tsv"
SUMMARY_FILE="${OUT_DIR}/SUMMARY.md"

grep -Eo 'https://developer\.nodit\.io/[^)[:space:]]+\.md' "${OUT_DIR}/llms.txt" \
  | sort -u > "${URL_LIST_FILE}"

doc_count="$(wc -l < "${URL_LIST_FILE}" | tr -d ' ')"
echo "Discovered ${doc_count} markdown pages"

: > "${MANIFEST_FILE}"

while IFS= read -r url; do
  rel_path="${url#${BASE_URL}/}"
  dest_path="${OUT_DIR}/raw/${rel_path}"
  mkdir -p "$(dirname "${dest_path}")"
  curl -fsSL "${url}" -o "${dest_path}"
  printf '%s\t%s\n' "${url}" "${dest_path#${ROOT_DIR}/}" >> "${MANIFEST_FILE}"
done < "${URL_LIST_FILE}"

guide_count="$(grep -c '/guides/' "${URL_LIST_FILE}" || true)"
api_count="$(grep -c '/api/' "${URL_LIST_FILE}" || true)"

cat > "${SUMMARY_FILE}" <<EOF
# Nodit Docs Snapshot

- Source: ${BASE_URL}
- Synced at (UTC): $(date -u +"%Y-%m-%dT%H:%M:%SZ")
- Markdown pages: ${doc_count}
- Guides: ${guide_count}
- API docs: ${api_count}

## Files

- \`llms.txt\`: discovery index published by Nodit
- \`sitemap.xml\`: full site URL map
- \`urls.txt\`: markdown page URLs extracted from \`llms.txt\`
- \`manifest.tsv\`: URL to local path mapping
- \`raw/\`: downloaded markdown source files preserving remote paths

## Refresh

\`\`\`bash
bash scripts/sync_nodit_docs.sh
\`\`\`
EOF

echo "Saved docs under ${OUT_DIR}"
