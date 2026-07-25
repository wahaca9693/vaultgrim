#!/bin/bash
# ============================================================
#  🔐 Vaultgrim - التثبيت الكامل من الكود المصدري
#  بدون الحاجة لـ GitHub Releases
# ============================================================

set -e

REPO="https://github.com/wahaca9693/vaultgrim.git"
INSTALL_DIR="${HOME}/vaultgrim"

# الألوان
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo ""
echo -e "${CYAN}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC}        ${GREEN}🔐 Vaultgrim - التثبيت الشامل${NC}               ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}        ${YELLOW}من الكود المصدري - بدون Releases${NC}           ${CYAN}║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# ============================================================
# 1. تثبيت Rust
# ============================================================
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}[1/5]${NC} جاري تثبيت Rust..."

if command -v cargo &> /dev/null; then
    echo -e "${GREEN}✅ Rust موجود مسبقاً: $(cargo --version)${NC}"
else
    echo "جاري تثبيت Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # تحميل البيئة
    if [ -f "${HOME}/.cargo/env" ]; then
        source "${HOME}/.cargo/env"
    fi
    
    echo -e "${GREEN}✅ تم تثبيت Rust${NC}"
fi

# ============================================================
# 2. تحميل الكود المصدري
# ============================================================
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}[2/5]${NC} جاري تحميل الكود المصدري..."

if [ -d "${INSTALL_DIR}" ]; then
    echo "المجلد موجود مسبقاً، جاري التحديث..."
    cd "${INSTALL_DIR}"
    git pull origin main
else
    echo "جاري استنساخ المستودع..."
    git clone "${REPO}" "${INSTALL_DIR}"
    cd "${INSTALL_DIR}"
fi

echo -e "${GREEN}✅ تم تحميل الكود المصدري${NC}"

# ============================================================
# 3. تهيئة Mirror لـ crates.io
# ============================================================
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}[3/5]${NC} جاري تهيئة إعدادات Cargo..."

if [ ! -f "${HOME}/.cargo/config.toml" ]; then
    mkdir -p "${HOME}/.cargo"
    cat > "${HOME}/.cargo/config.toml" << 'EOF'
[source.crates-io]
replace-with = "ustc"

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
EOF
    echo -e "${GREEN}✅ تم تهيئة Mirror (USTC)${NC}"
else
    echo -e "${GREEN}✅ Mirror موجود مسبقاً${NC}"
fi

# ============================================================
# 4. بناء الأداة
# ============================================================
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}[4/5]${NC} جاري بناء الأداة..."

source "${HOME}/.cargo/env" 2>/dev/null || true

# البناء مع features
cargo build --release --features pqc 2>&1

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ تم بناء الأداة بنجاح!${NC}"
else
    echo -e "${RED}❌ فشل البناء${NC}"
    exit 1
fi

# ============================================================
# 5. الاختبار
# ============================================================
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}[5/5]${NC} جاري اختبار الأداة..."

VAULTGRIM_PATH="${INSTALL_DIR}/target/release/vaultgrim"

if [ -f "${VAULTGRIM_PATH}" ]; then
    chmod +x "${VAULTGRIM_PATH}"
    echo -e "${GREEN}✅ الأداة جاهزة!${NC}"
    echo ""
    echo -e "${CYAN}╔════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║${NC}                  ${GREEN}✅ تم التثبيت بنجاح!${NC}                   ${CYAN}║${NC}"
    echo -e "${CYAN}╚════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "النسخة: $("${VAULTGRIM_PATH}" --version 2>/dev/null || echo '1.0.0')"
    echo "الموقع: ${VAULTGRIM_PATH}"
    echo ""
    
    # عرض الأوامر المتاحة
    echo -e "${YELLOW}الأوامر المتاحة:${NC}"
    echo ""
    "${VAULTGRIM_PATH}" --help
else
    echo -e "${RED}❌ لم يتم العثور على الأداة${NC}"
    exit 1
fi

# ============================================================
# إنشاء Alias
# ============================================================
echo ""
echo -e "${YELLOW}جاري إنشاء alias...${NC}"

if command -v zsh &> /dev/null; then
    RC="${HOME}/.zshrc"
else
    RC="${HOME}/.bashrc"
fi

ALIAS_LINE="alias vaultgrim='${VAULTGRIM_PATH}'"

if ! grep -q "vaultgrim=" "${RC}" 2>/dev/null; then
    echo "" >> "${RC}"
    echo "# Vaultgrim" >> "${RC}"
    echo "${ALIAS_LINE}" >> "${RC}"
    echo -e "${GREEN}✅ تم إنشاء alias في ${RC}${NC}"
else
    echo -e "${GREEN}✅ Alias موجود مسبقاً${NC}"
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}              🎉 التثبيت اكتمل! 🎉${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "للاستخدام فوراً، اكتب:"
echo -e "  source ${RC}"
echo "  vaultgrim --help"
echo ""
echo -e "أو مباشرة:"
echo -e "  ${VAULTGRIM_PATH} --help"
echo ""
