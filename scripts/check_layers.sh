#!/bin/bash
set -e

echo "--- Layer Dependency Check (Direct Dependencies Only) ---"

FAILED=0

check_no_direct_dependency() {
    local target=$1
    local forbidden=$2
    
    echo "Checking $target..."
    # --depth 1 オプションを使用して直接の依存関係のみをリストアップ
    local result=$(cargo tree -p "$target" --depth 1 --prefix none --no-dedupe | grep -E "^($forbidden) v" || true)
    
    if [ -n "$result" ]; then
        echo "  [ERROR] $target has prohibited DIRECT dependencies:"
        echo "$result" | sed 's/^/    /'
        FAILED=1
    else
        echo "  [OK] No prohibited direct dependencies found."
    fi
}

# 1. domain 層のチェック (自分より上位、または同等のレイヤーへの依存禁止)
check_no_direct_dependency "domain" "usecase|infrastructure|api|server"

# 2. usecase 層のチェック (実装層やエントリーポイントへの依存禁止)
check_no_direct_dependency "usecase" "infrastructure|api|server"

# 3. api 層のチェック (インフラ層や他アプリへの依存禁止)
# ※ 方針に従い、domain への直接依存も禁止
check_no_direct_dependency "api" "infrastructure|server|domain"

# 4. infrastructure 層のチェック (エントリーポイントへの依存禁止)
check_no_direct_dependency "infrastructure" "api|server"

if [ $FAILED -eq 1 ]; then
    echo "--- Check FAILED ---"
    exit 1
else
    echo "--- Check PASSED ---"
    exit 0
fi
