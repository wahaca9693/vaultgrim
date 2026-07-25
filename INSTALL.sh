#!/bin/bash
# ============================================================
#  🔐 Vaultgrim - التثبيت الشامل لجميع الأنظمة
# ============================================================

set -e

VERSION="1.0.0"
REPO="wahaca9693/vaultgrim"
INSTALL_DIR="${HOME}/.vaultgrim"
BIN_DIR="${INSTALL_DIR}/bin"

# الألوان
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════╗"
echo "║     🔐 Vaultgrim - أداة التشفير النهائية      ║"
echo "╚══════════════════════════════════════════════════╝"
echo -e "${NC}"

# كشف النظام
detect_os() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux*)     OS_NAME="linux" ;;
        Darwin*)    OS_NAME="macos" ;;
        CYGWIN*|MINGW*|MSYS*) OS_NAME="windows" ;;
        *)          echo "نظام غير مدعوم: $OS"; exit 1 ;;
    esac
    
    case "$ARCH" in
        x86_64)     CPU="x64" ;;
        aarch64|arm64) CPU="arm64" ;;
        armv7l)     CPU="arm" ;;
        *)          CPU="$ARCH" ;;
    esac
    
    BINARY_NAME="vaultgrim-${OS_NAME}-${CPU}"
}

# كشف طريقة التثبيت
detect_install_method() {
    echo -e "${YELLOW}جاري الكشف عن النظام...${NC}"
    detect_os
    echo "النظام: $OS_NAME | المعالج: $CPU"
    echo "الاسم: $BINARY_NAME"
}

# التحقق من وجود الأداة
check_prerequisites() {
    echo -e "\n${YELLOW}جاري التحقق من المتطلبات...${NC}"
    
    if command -v curl &> /dev/null; then
        echo "✅ curl موجود"
    else
        echo "❌ curl غير موجود"
        exit 1
    fi
}

# تثبيت من GitHub Releases
install_from_github() {
    echo -e "\n${YELLOW}جاري تحميل Vaultgrim...${NC}"
    
    mkdir -p "${BIN_DIR}"
    
    # اختيار الملف المناسب
    if [ "$OS_NAME" = "windows" ]; then
        URL="https://github.com/${REPO}/releases/latest/download/vaultgrim-${OS_NAME}-${CPU}.exe"
        DEST="${BIN_DIR}/vaultgrim.exe"
    else
        URL="https://github.com/${REPO}/releases/latest/download/vaultgrim-${OS_NAME}-${CPU}"
        DEST="${BIN_DIR}/vaultgrim"
    fi
    
    echo "الرابط: $URL"
    echo "الوجهة: $DEST"
    
    if curl -sL "$URL" -o "$DEST"; then
        chmod +x "$DEST"
        echo -e "${GREEN}✅ تم التحميل بنجاح!${NC}"
    else
        echo -e "${RED}❌ فشل التحميل${NC}"
        echo "جرب التثبيت من الكود المصدري..."
        install_from_source
        return
    fi
    
    # إنشاء رابط رمزي
    echo -e "\n${YELLOW}جاري إنشاء رابط رمزي...${NC}"
    ln -sf "${DEST}" "${INSTALL_DIR}/vaultgrim" 2>/dev/null || true
    
    # إضافة إلى PATH
    add_to_path
}

# التثبيت من الكود المصدري
install_from_source() {
    echo -e "\n${YELLOW}جاري التثبيت من الكود المصدري...${NC}"
    
    # التحقق من Rust
    if ! command -v cargo &> /dev/null; then
        echo "جاري تثبيت Rust..."
        if [ "$OS_NAME" = "macos" ]; then
            brew install rust
        elif [ "$OS_NAME" = "linux" ]; then
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source "${HOME}/.cargo/env" 2>/dev/null || true
        fi
    fi
    
    # تهيئة Mirror إذاneeded
    if [ ! -f "${HOME}/.cargo/config.toml" ]; then
        mkdir -p "${HOME}/.cargo"
        cat > "${HOME}/.cargo/config.toml" << 'EOF'
[source.crates-io]
replace-with = "ustc"

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
EOF
        echo "✅ تم تهيئة Mirror للـ crates.io"
    fi
    
    # استنساخ المشروع
    cd "${HOME}"
    if [ ! -d "vaultgrim-source" ]; then
        git clone "https://github.com/${REPO}.git" vaultgrim-source
    fi
    cd vaultgrim-source
    
    # البناء
    source "${HOME}/.cargo/env" 2>/dev/null || true
    cargo build --release
    
    # نسخ الملف
    mkdir -p "${BIN_DIR}"
    cp target/release/vaultgrim "${BIN_DIR}/"
    chmod +x "${BIN_DIR}/vaultgrim"
    
    echo -e "${GREEN}✅ تم البناء بنجاح!${NC}"
}

# إضافة إلى PATH
add_to_path() {
    echo -e "\n${YELLOW}جاري إضافة Vaultgrim إلى PATH...${NC}"
    
    SHELL_RC="${HOME}/.bashrc"
    if [ "$OS_NAME" = "macos" ]; then
        SHELL_RC="${HOME}/.zshrc"
    fi
    
    PATH_LINE="export PATH=\"\${PATH}:${BIN_DIR}\""
    
    if ! grep -q "${BIN_DIR}" "${SHELL_RC}" 2>/dev/null; then
        echo "" >> "${SHELL_RC}"
        echo "# Vaultgrim" >> "${SHELL_RC}"
        echo "${PATH_LINE}" >> "${SHELL_RC}"
        echo -e "${GREEN}✅ تم إضافة PATH إلى ${SHELL_RC}${NC}"
    else
        echo "✅ PATH موجود مسبقاً"
    fi
}

# الاختبار
test_install() {
    echo -e "\n${YELLOW}جاري اختبار التثبيت...${NC}"
    
    if [ -f "${BIN_DIR}/vaultgrim" ] || [ -f "${BIN_DIR}/vaultgrim.exe" ]; then
        "${BIN_DIR}/vaultgrim" --version 2>/dev/null || "${BIN_DIR}/vaultgrim.exe" --version 2>/dev/null
    else
        echo "❌ لم يتم العثور على الملف التنفيذي"
        exit 1
    fi
}

# التشغيل
run_demo() {
    echo -e "\n${GREEN}═══════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}✅ تم التثبيت بنجاح!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
    
    echo ""
    echo "للاستخدام، اكتب:"
    echo "  vaultgrim --help"
    echo ""
    echo "أو:"
    echo "  ${BIN_DIR}/vaultgrim --help"
    echo ""
    
    # تشغيل عرض توضيحي
    read -p "هل تريد تشغيل عرض توضيحي سريع? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cd "${HOME}"
        echo "Hello Vaultgrim!" > test_file.txt
        "${BIN_DIR}/vaultgrim" encrypt test_file.txt 2>/dev/null || "${BIN_DIR}/vaultgrim.exe" encrypt test_file.txt 2>/dev/null
        echo ""
        echo "✅ انتهى العرض التوضيحي!"
        rm -f test_file.txt* 2>/dev/null
    fi
}

# ============================================================
#  بدء التثبيت
# ============================================================
main() {
    detect_install_method
    check_prerequisites
    install_from_github
    test_install
    run_demo
    
    echo -e "\n${BLUE}شكراً لاستخدام Vaultgrim! 🔐${NC}\n"
}

main "$@"
