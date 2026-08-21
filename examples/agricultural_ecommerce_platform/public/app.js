// ========================================================================
// Nông Sản Việt — Enterprise Frontend Single Page Application (VietLang Powered)
// 100% Pure Relational REST API Client + Full Auth & RBAC Permissions
// ========================================================================

const state = {
  currentUser: JSON.parse(localStorage.getItem('nsv_user') || 'null'),
  categories: [],
  cooperatives: [],
  vouchers: [],
  products: [],
  users: [],
  selectedCategory: 0,
  selectedRegion: "",
  selectedCert: "",
  sortBy: "default",
  searchQuery: "",
  cart: [],
  voucher: null,
  orders: []
};

// Toast Notification Engine
function showToast(message, type = 'success') {
  const toast = document.createElement('div');
  toast.className = `toast-popup ${type}`;
  toast.innerHTML = `<span>${message}</span>`;
  document.body.appendChild(toast);
  setTimeout(() => toast.classList.add('visible'), 50);
  setTimeout(() => {
    toast.classList.remove('visible');
    setTimeout(() => toast.remove(), 300);
  }, 3200);
}

// Currency Formatting Helper
function formatVND(amount) {
  return new Intl.NumberFormat('vi-VN').format(amount || 0) + ' VNĐ';
}

// Copy Voucher Code
window.copyVoucher = function(code) {
  navigator.clipboard.writeText(code).then(() => {
    showToast(`Đã sao chép mã voucher [${code}]!`);
    const inp = document.getElementById('voucherInput');
    if (inp) inp.value = code;
  }).catch(() => {
    const inp = document.getElementById('voucherInput');
    if (inp) inp.value = code;
    showToast(`Đã chọn mã: ${code}`);
  });
};

// DOM Elements
const productsGrid = document.getElementById('productsGrid');
const productCounter = document.getElementById('productCounter');
const categoryTabs = document.getElementById('categoryTabs');
const regionSelect = document.getElementById('regionSelect');
const certSelect = document.getElementById('certSelect');
const sortSelect = document.getElementById('sortSelect');
const searchInput = document.getElementById('searchInput');
const searchBtn = document.getElementById('searchBtn');
const flashSaleGrid = document.getElementById('flashSaleGrid');
const cooperativesGrid = document.getElementById('cooperativesGrid');

const cartToggleBtn = document.getElementById('cartToggleBtn');
const cartDrawer = document.getElementById('cartDrawer');
const cartBackdrop = document.getElementById('cartBackdrop');
const cartBadge = document.getElementById('cartBadge');
const cartItemsList = document.getElementById('cartItemsList');
const cartSubtotalText = document.getElementById('cartSubtotalText');
const cartDiscountText = document.getElementById('cartDiscountText');
const cartShippingText = document.getElementById('cartShippingText');
const cartTotalText = document.getElementById('cartTotalText');
const openCheckoutBtn = document.getElementById('openCheckoutBtn');
const voucherInput = document.getElementById('voucherInput');
const applyVoucherBtn = document.getElementById('applyVoucherBtn');

const authModal = document.getElementById('authModal');
const openAuthBtn = document.getElementById('openAuthBtn');
const userProfileBadge = document.getElementById('userProfileBadge');
const navUserName = document.getElementById('navUserName');
const navUserRole = document.getElementById('navUserRole');
const logoutBtn = document.getElementById('logoutBtn');
const loginForm = document.getElementById('loginForm');
const registerForm = document.getElementById('registerForm');
const tabLoginBtn = document.getElementById('tabLoginBtn');
const tabRegisterBtn = document.getElementById('tabRegisterBtn');

const productDetailModal = document.getElementById('productDetailModal');
const modalDetailContent = document.getElementById('modalDetailContent');

const checkoutModal = document.getElementById('checkoutModal');
const checkoutForm = document.getElementById('checkoutForm');

const trackModal = document.getElementById('trackModal');
const navTrackOrderBtn = document.getElementById('navTrackOrderBtn');
const trackInput = document.getElementById('trackInput');
const doTrackBtn = document.getElementById('doTrackBtn');
const trackResultBox = document.getElementById('trackResultBox');

const adminModal = document.getElementById('adminModal');
const navAdminBtn = document.getElementById('navAdminBtn');
const admTotalRev = document.getElementById('admTotalRev');
const admTotalOrders = document.getElementById('admTotalOrders');
const admTotalProds = document.getElementById('admTotalProds');
const admLowStockCount = document.getElementById('admLowStockCount');
const inventoryTableBody = document.getElementById('inventoryTableBody');
const ordersTableBody = document.getElementById('ordersTableBody');
const usersTableBody = document.getElementById('usersTableBody');
const addProductForm = document.getElementById('addProductForm');

// ========================================================================
// 1. Authentication & Role-Based Access Control (RBAC)
// ========================================================================
function renderUserAuthUI() {
  if (state.currentUser) {
    if (openAuthBtn) openAuthBtn.style.display = 'none';
    if (userProfileBadge) {
      userProfileBadge.style.display = 'flex';
      navUserName.textContent = state.currentUser.name || 'Người Dùng';
      navUserRole.textContent = state.currentUser.role || 'CUSTOMER';
      if (state.currentUser.role === 'ADMIN') {
        navUserRole.className = 'cert-badge cert-ocop';
      } else if (state.currentUser.role === 'FARMER') {
        navUserRole.className = 'cert-badge cert-vietgap';
      } else {
        navUserRole.className = 'cert-badge cert-organic';
      }
    }
  } else {
    if (openAuthBtn) openAuthBtn.style.display = 'block';
    if (userProfileBadge) userProfileBadge.style.display = 'none';
  }
}

if (openAuthBtn) {
  openAuthBtn.addEventListener('click', () => {
    authModal.classList.add('open');
  });
}

window.closeAuthModal = function() {
  if (authModal) authModal.classList.remove('open');
};

window.switchAuthTab = function(tab) {
  if (tab === 'login') {
    tabLoginBtn.className = 'btn btn-primary btn-sm';
    tabRegisterBtn.className = 'btn btn-secondary btn-sm';
    loginForm.style.display = 'block';
    registerForm.style.display = 'none';
    document.getElementById('authModalTitle').textContent = 'Đăng Nhập Tài Khoản';
  } else {
    tabLoginBtn.className = 'btn btn-secondary btn-sm';
    tabRegisterBtn.className = 'btn btn-primary btn-sm';
    loginForm.style.display = 'none';
    registerForm.style.display = 'block';
    document.getElementById('authModalTitle').textContent = 'Đăng Ký Tài Khoản Mới';
  }
};

window.quickLogin = function(email, password) {
  document.getElementById('loginEmail').value = email;
  document.getElementById('loginPassword').value = password;
  loginForm.dispatchEvent(new Event('submit'));
};

if (loginForm) {
  loginForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const email = document.getElementById('loginEmail').value.trim();
    const password = document.getElementById('loginPassword').value;

    showToast('Đang xác thực thông tin đăng nhập...', 'info');

    try {
      const res = await fetch('/api/v1/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password })
      });

      const result = await res.json();
      if (!res.ok || result.status_code >= 400) {
        showToast(result.error || 'Đăng nhập thất bại!', 'error');
        return;
      }

      const u = result.data.user;
      u.token = result.data.token;
      state.currentUser = u;
      localStorage.setItem('nsv_user', JSON.stringify(u));

      renderUserAuthUI();
      closeAuthModal();
      showToast(`Chào mừng [${u.name}] (Vai trò: ${u.role}) đăng nhập thành công!`);

    } catch (err) {
      showToast('Lỗi kết nối tới máy chủ: ' + err.message, 'error');
    }
  });
}

if (registerForm) {
  registerForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const name = document.getElementById('regName').value.trim();
    const email = document.getElementById('regEmail').value.trim();
    const phone = document.getElementById('regPhone').value.trim();
    const password = document.getElementById('regPassword').value;
    const role = document.getElementById('regRole').value;

    showToast('Đang đăng ký tài khoản vào CSDL...', 'info');

    try {
      const res = await fetch('/api/v1/auth/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, email, phone, password, role })
      });

      const result = await res.json();
      if (!res.ok || result.status_code >= 400) {
        showToast(result.error || 'Đăng ký thất bại!', 'error');
        return;
      }

      const u = result.data.user;
      u.token = result.data.token;
      state.currentUser = u;
      localStorage.setItem('nsv_user', JSON.stringify(u));

      renderUserAuthUI();
      closeAuthModal();
      showToast(`Tạo tài khoản thành công! Quyền hạn: ${u.role}`);

    } catch (err) {
      showToast('Lỗi kết nối: ' + err.message, 'error');
    }
  });
}

if (logoutBtn) {
  logoutBtn.addEventListener('click', () => {
    state.currentUser = null;
    localStorage.removeItem('nsv_user');
    renderUserAuthUI();
    showToast('Đã đăng xuất khỏi hệ thống!');
  });
}

// ========================================================================
// 2. Initial Data Fetching from Pure VietLang Relational Backend
// ========================================================================
async function fetchInitialData() {
  try {
    const [catRes, coopRes, vRes, prodRes] = await Promise.all([
      fetch('/api/v1/categories'),
      fetch('/api/v1/cooperatives'),
      fetch('/api/v1/vouchers'),
      fetch('/api/v1/products')
    ]);

    if (catRes.ok) {
      const catJson = await catRes.json();
      if (catJson && catJson.data) {
        state.categories = catJson.data;
        renderCategoryTabs();
      }
    }

    if (coopRes.ok) {
      const coopJson = await coopRes.json();
      if (coopJson && coopJson.data) {
        state.cooperatives = coopJson.data;
        renderCooperatives();
      }
    }

    if (vRes.ok) {
      const vJson = await vRes.json();
      if (vJson && vJson.data) {
        state.vouchers = vJson.data;
      }
    }

    if (prodRes.ok) {
      const prodJson = await prodRes.json();
      if (prodJson && prodJson.data) {
        state.products = prodJson.data;
        renderProducts();
        renderFlashSale();
      }
    }

    renderUserAuthUI();
  } catch (err) {
    console.error('[VietLang Backend Error]', err);
  }
}

// Render Category Tabs Dynamically
function renderCategoryTabs() {
  if (!categoryTabs) return;
  categoryTabs.innerHTML = '';

  const allBtn = document.createElement('button');
  allBtn.className = `category-tab ${state.selectedCategory === 0 ? 'active' : ''}`;
  allBtn.innerHTML = '<span>Tất Cả Nông Sản</span>';
  allBtn.onclick = () => {
    state.selectedCategory = 0;
    document.querySelectorAll('.category-tab').forEach(t => t.classList.remove('active'));
    allBtn.classList.add('active');
    renderProducts();
  };
  categoryTabs.appendChild(allBtn);

  state.categories.forEach(cat => {
    const btn = document.createElement('button');
    btn.className = `category-tab ${state.selectedCategory === cat.id ? 'active' : ''}`;
    btn.innerHTML = `<span>${cat.name}</span>`;
    btn.onclick = () => {
      state.selectedCategory = cat.id;
      document.querySelectorAll('.category-tab').forEach(t => t.classList.remove('active'));
      btn.classList.add('active');
      renderProducts();
    };
    categoryTabs.appendChild(btn);
  });
}

// ========================================================================
// 3. Products Grid Rendering
// ========================================================================
function renderProducts() {
  if (!productsGrid) return;
  let list = [...state.products];

  if (state.selectedCategory > 0) {
    list = list.filter(p => p.category_id === state.selectedCategory);
  }
  if (state.selectedRegion) {
    list = list.filter(p => (p.region || '').toLowerCase().includes(state.selectedRegion.toLowerCase()));
  }
  if (state.selectedCert) {
    list = list.filter(p => (p.cert || '').toLowerCase().includes(state.selectedCert.toLowerCase()));
  }
  if (state.searchQuery.trim() !== '') {
    const q = state.searchQuery.toLowerCase();
    list = list.filter(p => (p.name || '').toLowerCase().includes(q) || (p.origin || '').toLowerCase().includes(q) || (p.description || '').toLowerCase().includes(q));
  }

  if (state.sortBy === 'price_asc') {
    list.sort((a, b) => a.price - b.price);
  } else if (state.sortBy === 'price_desc') {
    list.sort((a, b) => b.price - a.price);
  } else if (state.sortBy === 'rating') {
    list.sort((a, b) => b.rating - a.rating);
  }

  productCounter.textContent = `${list.length} sản phẩm`;
  productsGrid.innerHTML = '';

  if (list.length === 0) {
    productsGrid.innerHTML = `
      <div style="grid-column: 1 / -1; text-align: center; padding: 60px 20px; background: var(--bg-surface); border-radius: var(--radius-lg); border: 1px dashed var(--border-subtle);">
        <h3 style="color: var(--text-muted); margin-bottom: 8px;">Không tìm thấy nông sản phù hợp</h3>
        <p style="color: var(--text-dim); font-size: 14px;">Vui lòng điều chỉnh lại bộ lọc danh mục, vùng miền hoặc từ khóa tìm kiếm.</p>
      </div>
    `;
    return;
  }

  list.forEach(p => {
    const card = document.createElement('div');
    card.className = 'product-card';
    card.id = `prod_${p.id}`;

    const discountPercent = p.original_price > p.price ? Math.round(((p.original_price - p.price) / p.original_price) * 100) : 0;
    const certClass = (p.cert || '').includes('5 Sao') ? 'cert-ocop' : (p.cert || '').includes('VietGAP') ? 'cert-vietgap' : 'cert-organic';

    card.innerHTML = `
      <div class="product-media" onclick="openProductDetail(${p.id})">
        ${p.image_data || "<div style='width:100%;height:200px;background:#182C1F;display:flex;align-items:center;justify-content:center;color:#10B981;font-weight:700;'>NÔNG SẢN VIỆT</div>"}
        <span class="product-badge-cert ${certClass}">${p.cert || 'Chuẩn OCOP'}</span>
        ${discountPercent > 0 ? `<span class="product-badge-discount">-${discountPercent}%</span>` : ''}
      </div>
      <div class="product-body">
        <div class="product-origin">${p.origin || 'Việt Nam'} · Thu hoạch: ${p.harvest_date || 'Mới'}</div>
        <h3 class="product-title" onclick="openProductDetail(${p.id})">${p.name}</h3>
        <div class="product-price-row">
          <div class="price-current">${formatVND(p.price)} <small>/ ${p.unit}</small></div>
          ${p.original_price > p.price ? `<div class="price-original">${formatVND(p.original_price)}</div>` : ''}
        </div>
        <div class="stock-progress-wrap">
          <div class="stock-info">
            <span>Tồn kho: <strong>${p.stock} ${p.unit}</strong></span>
            <span>⭐ ${p.rating || 5.0} (${p.review_count || 0})</span>
          </div>
          <div class="stock-progress-bar">
            <div class="stock-progress-fill" style="width: ${Math.min(100, Math.max(10, (p.stock / 500) * 100))}%"></div>
          </div>
        </div>
        <div class="product-actions">
          <button class="btn btn-secondary btn-sm" onclick="openProductDetail(${p.id})">Chi Tiết</button>
          <button class="btn btn-primary btn-sm" onclick="addToCart(${p.id})">+ Thêm Giỏ</button>
        </div>
      </div>
    `;
    productsGrid.appendChild(card);
  });
}

// Render Flash Sale Deals Grid
function renderFlashSale() {
  if (!flashSaleGrid) return;
  const saleProducts = state.products.filter(p => (p.original_price - p.price) >= 20000).slice(0, 4);
  flashSaleGrid.innerHTML = '';

  saleProducts.forEach(p => {
    const card = document.createElement('div');
    card.className = 'product-card flash-deal-card';
    const discount = Math.round(((p.original_price - p.price) / p.original_price) * 100);

    card.innerHTML = `
      <div class="product-media" onclick="openProductDetail(${p.id})">
        ${p.image_data || ""}
        <span class="product-badge-discount">🔥 -${discount}%</span>
      </div>
      <div class="product-body">
        <div class="product-origin">${p.origin} · ${p.cert}</div>
        <h3 class="product-title" onclick="openProductDetail(${p.id})">${p.name}</h3>
        <div class="product-price-row">
          <div class="price-current" style="color: var(--color-gold);">${formatVND(p.price)}</div>
          <div class="price-original">${formatVND(p.original_price)}</div>
        </div>
        <button class="btn btn-primary btn-sm" style="width: 100%; margin-top: 10px;" onclick="addToCart(${p.id})">Săn Deal Ngay</button>
      </div>
    `;
    flashSaleGrid.appendChild(card);
  });
}

// Render Cooperatives Grid
function renderCooperatives() {
  if (!cooperativesGrid) return;
  cooperativesGrid.innerHTML = '';

  state.cooperatives.forEach(c => {
    const card = document.createElement('div');
    card.className = 'coop-card';
    card.innerHTML = `
      <div class="coop-badge">${c.cert}</div>
      <h3 class="coop-name">${c.name}</h3>
      <div class="coop-location">Địa bàn: ${c.location} (Thành lập: ${c.founded_year})</div>
      <div class="coop-stats">
        <div class="coop-stat-item">
          <strong>${c.hectares} ha</strong>
          <span>Vùng canh tác</span>
        </div>
        <div class="coop-stat-item">
          <strong>${c.farmer_count}</strong>
          <span>Hộ Xã Viên</span>
        </div>
      </div>
      <div style="margin-top: 14px; font-size: 13px; color: var(--text-dim);">
        Hotline HTX: <strong style="color: var(--color-primary-light);">${c.contact}</strong>
      </div>
    `;
    cooperativesGrid.appendChild(card);
  });
}

// Flash Sale Timer
function initFlashSaleTimer() {
  let secondsLeft = 14 * 3600 + 42 * 60 + 19;
  const hoursEl = document.getElementById('flashHours');
  const minsEl = document.getElementById('flashMinutes');
  const secsEl = document.getElementById('flashSeconds');

  setInterval(() => {
    if (secondsLeft <= 0) secondsLeft = 24 * 3600;
    secondsLeft--;

    const h = Math.floor(secondsLeft / 3600);
    const m = Math.floor((secondsLeft % 3600) / 60);
    const s = secondsLeft % 60;

    if (hoursEl) hoursEl.textContent = String(h).padStart(2, '0');
    if (minsEl) minsEl.textContent = String(m).padStart(2, '0');
    if (secsEl) secsEl.textContent = String(s).padStart(2, '0');
  }, 1000);
}

// ========================================================================
// 4. Cart Mechanics
// ========================================================================
window.toggleCart = function() {
  cartDrawer.classList.toggle('open');
  cartBackdrop.classList.toggle('open');
};
cartToggleBtn.addEventListener('click', toggleCart);
cartBackdrop.addEventListener('click', toggleCart);

window.addToCart = function(productId) {
  const prod = state.products.find(p => p.id === productId);
  if (!prod) return;

  if (prod.stock <= 0) {
    showToast(`Sản phẩm [${prod.name}] tạm thời hết hàng!`, 'error');
    return;
  }

  const existing = state.cart.find(item => item.product_id === productId);
  if (existing) {
    if (existing.quantity >= prod.stock) {
      showToast(`Số lượng trong giỏ đã đạt mức tồn kho tối đa (${prod.stock} ${prod.unit})!`, 'error');
      return;
    }
    existing.quantity += 1;
  } else {
    state.cart.push({
      product_id: prod.id,
      name: prod.name,
      price: prod.price,
      unit: prod.unit,
      quantity: 1,
      stock: prod.stock
    });
  }

  updateCartUI();
  showToast(`Đã thêm [${prod.name}] vào giỏ hàng!`);
};

window.updateCartQty = function(productId, delta) {
  const itemIndex = state.cart.findIndex(i => i.product_id === productId);
  if (itemIndex === -1) return;

  const item = state.cart[itemIndex];
  const prod = state.products.find(p => p.id === productId);

  item.quantity += delta;
  if (item.quantity <= 0) {
    state.cart.splice(itemIndex, 1);
  } else if (prod && item.quantity > prod.stock) {
    item.quantity = prod.stock;
    showToast(`Chỉ còn ${prod.stock} ${prod.unit} trong kho!`, 'error');
  }

  updateCartUI();
};

function calculateCartTotals() {
  const subtotal = state.cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
  let discount = 0;
  const shipping = subtotal >= 150000 || subtotal === 0 ? 0 : 30000;

  if (state.voucher && subtotal > 0) {
    if (state.voucher === 'NONGSANVIET20') {
      discount = Math.round(subtotal * 0.2);
    } else if (state.voucher === 'FREESHIP') {
      discount = shipping;
    } else if (state.voucher === 'HELLOTET' && subtotal >= 300000) {
      discount = 50000;
    } else if (state.voucher === 'OCOP10') {
      discount = Math.round(subtotal * 0.1);
    }
  }

  const total = Math.max(0, subtotal + shipping - discount);
  return { subtotal, discount, shipping, total };
}

function updateCartUI() {
  const totalCount = state.cart.reduce((sum, item) => sum + item.quantity, 0);
  cartBadge.textContent = totalCount;

  const { subtotal, discount, shipping, total } = calculateCartTotals();

  cartSubtotalText.textContent = formatVND(subtotal);
  cartDiscountText.textContent = `-${formatVND(discount)}`;
  cartShippingText.textContent = shipping === 0 ? (subtotal > 0 ? 'MIỄN PHÍ' : '0 VNĐ') : formatVND(shipping);
  cartTotalText.textContent = formatVND(total);

  cartItemsList.innerHTML = '';
  if (state.cart.length === 0) {
    cartItemsList.innerHTML = `
      <div style="text-align: center; padding: 40px 10px; color: var(--text-muted);">
        <p>Giỏ hàng của bạn đang trống</p>
        <button class="btn btn-secondary btn-sm" style="margin-top: 12px;" onclick="toggleCart()">Tiếp Tục Chọn Nông Sản</button>
      </div>
    `;
    openCheckoutBtn.disabled = true;
    return;
  }

  openCheckoutBtn.disabled = false;
  state.cart.forEach(item => {
    const row = document.createElement('div');
    row.className = 'cart-item';
    row.innerHTML = `
      <div class="cart-item-info">
        <h4 class="cart-item-title">${item.name}</h4>
        <div class="cart-item-price">${formatVND(item.price)} / ${item.unit}</div>
      </div>
      <div class="cart-item-qty">
        <button class="qty-btn" onclick="updateCartQty(${item.product_id}, -1)">-</button>
        <span class="qty-val">${item.quantity}</span>
        <button class="qty-btn" onclick="updateCartQty(${item.product_id}, 1)">+</button>
      </div>
    `;
    cartItemsList.appendChild(row);
  });
}

// Voucher Application
applyVoucherBtn.addEventListener('click', () => {
  const code = voucherInput.value.trim().toUpperCase();
  if (!code) {
    showToast('Vui lòng nhập mã giảm giá!', 'error');
    return;
  }

  const validCodes = ['NONGSANVIET20', 'FREESHIP', 'HELLOTET', 'OCOP10'];
  if (!validCodes.includes(code)) {
    showToast('Mã voucher không hợp lệ hoặc đã hết hạn!', 'error');
    return;
  }

  state.voucher = code;
  updateCartUI();
  showToast(`Áp dụng mã [${code}] thành công!`);
});

// Filters
regionSelect.addEventListener('change', (e) => {
  state.selectedRegion = e.target.value;
  renderProducts();
});
certSelect.addEventListener('change', (e) => {
  state.selectedCert = e.target.value;
  renderProducts();
});
sortSelect.addEventListener('change', (e) => {
  state.sortBy = e.target.value;
  renderProducts();
});

function handleSearch() {
  state.searchQuery = searchInput.value;
  renderProducts();
}
searchBtn.addEventListener('click', handleSearch);
searchInput.addEventListener('keyup', (e) => {
  if (e.key === 'Enter') handleSearch();
});

// Product Detail Modal
window.openProductDetail = async function(productId) {
  const prod = state.products.find(p => p.id === productId);
  if (!prod) return;

  const coop = state.cooperatives.find(c => c.id === prod.coop_id) || { name: 'HTX Nông Nghiệp Liên Kết', location: prod.origin };

  modalDetailContent.innerHTML = `
    <div style="display: grid; grid-template-columns: 1fr 1.2fr; gap: 24px;">
      <div>
        <div style="border-radius: var(--radius-md); overflow: hidden; border: 1px solid var(--border-subtle);">
          ${prod.image_data || ''}
        </div>
        <div style="margin-top: 16px; background: var(--bg-surface-elevated); padding: 12px; border-radius: var(--radius-md);">
          <div style="font-size: 12px; color: var(--text-muted);">HỢP TÁC XÃ SẢN XUẤT:</div>
          <strong style="color: var(--color-primary-light);">${coop.name}</strong>
          <div style="font-size: 12px; color: var(--text-dim); margin-top: 4px;">Địa bàn: ${coop.location}</div>
        </div>
      </div>
      <div>
        <span class="cert-badge">${prod.cert}</span>
        <h2 style="font-size: 20px; font-weight: 800; margin: 10px 0;">${prod.name}</h2>
        <div style="font-size: 22px; color: var(--color-primary-light); font-weight: 800; margin-bottom: 12px;">
          ${formatVND(prod.price)} <small style="font-size: 14px; color: var(--text-dim);">/ ${prod.unit}</small>
        </div>
        <p style="font-size: 14px; line-height: 1.6; color: var(--text-muted); margin-bottom: 16px;">${prod.description}</p>
        <div style="background: var(--bg-surface); padding: 12px; border-radius: var(--radius-md); font-size: 13px; margin-bottom: 16px;">
          <div>🌾 <strong>Xuất xứ:</strong> ${prod.origin}</div>
          <div style="margin-top: 4px;">📅 <strong>Thời vụ thu hoạch:</strong> ${prod.harvest_date}</div>
          <div style="margin-top: 4px;">📦 <strong>Tồn kho CSDL:</strong> ${prod.stock} ${prod.unit}</div>
        </div>
        <button class="btn btn-primary" style="width: 100%; padding: 14px;" onclick="addToCart(${prod.id}); closeProductDetail();">+ Thêm Vào Giỏ Hàng</button>
      </div>
    </div>
  `;

  productDetailModal.classList.add('open');
};

window.closeProductDetail = function() {
  productDetailModal.classList.remove('open');
};

// ========================================================================
// 5. Checkout Transaction (POST /api/v1/orders/checkout)
// ========================================================================
openCheckoutBtn.addEventListener('click', () => {
  const { total } = calculateCartTotals();
  if (total <= 0) {
    showToast('Giỏ hàng trống!', 'error');
    return;
  }

  // Pre-fill user profile if logged in
  if (state.currentUser) {
    document.getElementById('custName').value = state.currentUser.name || '';
    document.getElementById('custPhone').value = state.currentUser.phone || '';
  }

  toggleCart();
  checkoutModal.classList.add('open');
});

window.closeCheckoutModal = function() {
  checkoutModal.classList.remove('open');
};

checkoutForm.addEventListener('submit', async (e) => {
  e.preventDefault();

  const customer_name = document.getElementById('custName').value.trim();
  const phone = document.getElementById('custPhone').value.trim();
  const address = document.getElementById('custAddress').value.trim();
  const city = document.getElementById('custCity')?.value || 'TP. Hồ Chí Minh';
  const payment_method = document.getElementById('paymentMethod').value;

  const checkoutPayload = {
    user_id: state.currentUser ? state.currentUser.id : 2,
    customer_name,
    phone,
    address,
    city,
    payment_method,
    items: state.cart.map(i => ({ product_id: i.product_id, quantity: i.quantity })),
    voucher: state.voucher || ""
  };

  showToast('Đang gửi giao dịch ACID tới máy chủ VietLang...', 'info');

  try {
    const res = await fetch('/api/v1/orders/checkout', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(checkoutPayload)
    });

    const result = await res.json();
    if (!res.ok || result.status_code >= 400) {
      showToast(result.error || 'Đặt hàng thất bại!', 'error');
      return;
    }

    const orderData = result.data;
    const orderId = orderData.order_id || ('NSV' + Math.floor(100000 + Math.random() * 900000));

    state.cart = [];
    state.voucher = null;
    updateCartUI();
    closeCheckoutModal();

    await fetchInitialData();

    if (payment_method.includes('VIETQR')) {
      showVietQRModal({
        id: orderId,
        customer_name,
        phone,
        total: orderData.total || calculateCartTotals().total
      });
    } else {
      alert(`[GIAO DỊCH THÀNH CÔNG — CƠ SỞ DỮ LIỆU CHUẨN ACID]\n\nMã đơn hàng: #${orderId}\nKhách hàng: ${customer_name}\nTổng thanh toán: ${formatVND(orderData.total)}\n\nĐơn hàng đã được lưu trữ vĩnh viễn vào CSDL!`);
    }

  } catch (err) {
    showToast('Lỗi kết nối tới máy chủ VietLang: ' + err.message, 'error');
  }
});

// VietQR Modal
function showVietQRModal(order) {
  const qrDiv = document.createElement('div');
  qrDiv.className = 'modal-backdrop open';
  qrDiv.id = 'qrModalTemp';
  qrDiv.innerHTML = `
    <div class="modal-dialog" style="max-width: 480px; text-align: center;">
      <div class="modal-header">
        <h3 class="modal-title">Thanh Toán VietQR Tự Động</h3>
        <button class="modal-close" onclick="document.getElementById('qrModalTemp').remove()">&times;</button>
      </div>
      <div class="modal-body" style="padding: 24px;">
        <div style="background: white; padding: 16px; border-radius: 12px; display: inline-block; margin-bottom: 16px;">
          <svg viewBox="0 0 200 200" width="180" height="180">
            <rect width="200" height="200" fill="white"/>
            <rect x="20" y="20" width="50" height="50" fill="#064E3B"/>
            <rect x="30" y="30" width="30" height="30" fill="white"/>
            <rect x="40" y="40" width="10" height="10" fill="#064E3B"/>
            <rect x="130" y="20" width="50" height="50" fill="#064E3B"/>
            <rect x="140" y="30" width="30" height="30" fill="white"/>
            <rect x="150" y="40" width="10" height="10" fill="#064E3B"/>
            <rect x="20" y="130" width="50" height="50" fill="#064E3B"/>
            <rect x="30" y="140" width="30" height="30" fill="white"/>
            <rect x="40" y="150" width="10" height="10" fill="#064E3B"/>
            <rect x="80" y="30" width="20" height="40" fill="#10B981"/>
            <rect x="80" y="80" width="40" height="40" fill="#047857"/>
            <rect x="130" y="130" width="40" height="40" fill="#10B981"/>
          </svg>
        </div>
        <h4 style="color: var(--color-primary-light); font-size: 18px; margin-bottom: 8px;">Số tiền: ${formatVND(order.total)}</h4>
        <div style="background: var(--bg-surface-elevated); padding: 16px; border-radius: 8px; font-size: 13px; text-align: left; line-height: 1.8;">
          <p><strong>Ngân hàng:</strong> Techcombank (Chi nhánh Hà Nội)</p>
          <p><strong>Số tài khoản:</strong> 1903688899999</p>
          <p><strong>Chủ tài khoản:</strong> NONG SAN VIET ENTERPRISE</p>
          <p><strong>Nội dung:</strong> <span style="color: #F59E0B; font-weight: bold;">${order.id} ${order.phone}</span></p>
        </div>
        <button class="btn btn-primary" style="width: 100%; margin-top: 16px; padding: 12px;" onclick="document.getElementById('qrModalTemp').remove(); showToast('Hệ thống đã ghi nhận thanh toán!');">Xác Nhận Đã Chuyển Khoản</button>
      </div>
    </div>
  `;
  document.body.appendChild(qrDiv);
}

// ========================================================================
// 6. Order Tracking (GET /api/v1/orders/track)
// ========================================================================
navTrackOrderBtn.addEventListener('click', () => {
  trackModal.classList.add('open');
});

window.closeTrackModal = function() {
  trackModal.classList.remove('open');
};

doTrackBtn.addEventListener('click', async () => {
  const query = trackInput.value.trim().toUpperCase();
  if (!query) {
    trackResultBox.innerHTML = '<p style="color: var(--color-danger);">Vui lòng nhập số điện thoại hoặc mã đơn hàng để tra cứu!</p>';
    return;
  }

  trackResultBox.innerHTML = '<p style="color: var(--text-muted); text-align: center;">Đang truy vấn CSDL...</p>';

  try {
    const res = await fetch(`/api/v1/orders/track`);
    if (res.ok) {
      const json = await res.json();
      const allOrders = json.data || [];
      const matches = allOrders.filter(o => String(o.id).toUpperCase().includes(query) || String(o.phone).includes(query));

      trackResultBox.innerHTML = '';
      if (matches.length === 0) {
        trackResultBox.innerHTML = `
          <div style="background: var(--bg-surface-elevated); padding: 16px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle);">
            <p style="color: var(--text-muted); font-size: 14px;">Không tìm thấy đơn hàng trong CSDL cho thông tin: <strong>${query}</strong></p>
          </div>
        `;
        return;
      }

      matches.forEach(o => {
        const div = document.createElement('div');
        div.style = 'background: var(--bg-surface-elevated); padding: 16px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle); margin-bottom: 12px;';
        div.innerHTML = `
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
            <h4 style="color: var(--color-primary-light);">Đơn Hàng #${o.id}</h4>
            <span class="cert-badge">${o.status}</span>
          </div>
          <p style="font-size: 13px; color: var(--text-muted); margin-bottom: 4px;">Khách hàng: <strong>${o.customer_name}</strong> — ${o.phone}</p>
          <p style="font-size: 13px; color: var(--text-muted); margin-bottom: 8px;">Địa chỉ: ${o.address || ''} (${o.city || ''})</p>
          <div style="border-top: 1px solid var(--border-subtle); padding-top: 8px; font-size: 13px;">
            <p>Tổng thanh toán: <strong style="color: var(--color-primary-light);">${formatVND(o.total)}</strong> (${o.payment_method})</p>
          </div>
        `;
        trackResultBox.appendChild(div);
      });
    }
  } catch (err) {
    trackResultBox.innerHTML = `<p style="color: var(--color-danger);">Lỗi truy vấn: ${err.message}</p>`;
  }
});

// ========================================================================
// 7. Protected Admin Portal & RBAC Management Console
// ========================================================================
navAdminBtn.addEventListener('click', () => {
  // RBAC Permission Check
  if (!state.currentUser) {
    showToast('Vui lòng đăng nhập với tài khoản [ADMIN] để vào Cổng Quản Trị!', 'error');
    authModal.classList.add('open');
    return;
  }

  if (state.currentUser.role !== 'ADMIN') {
    showToast(`Từ chối truy cập! Tài khoản [${state.currentUser.name}] có vai trò (${state.currentUser.role}), yêu cầu quyền (ADMIN)!`, 'error');
    return;
  }

  document.getElementById('adminRoleBadge').textContent = `ADMIN ACCESS: ${state.currentUser.name}`;
  updateAdminAnalytics();
  loadAdminUsers();
  adminModal.classList.add('open');
});

window.closeAdminModal = function() {
  adminModal.classList.remove('open');
};

window.switchAdminTab = function(tabName) {
  document.querySelectorAll('.admin-tab').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('.admin-tab-content').forEach(c => c.classList.remove('active'));

  if (tabName === 'stats') {
    document.querySelector('.admin-tab:nth-child(1)').classList.add('active');
    document.getElementById('tabStats').classList.add('active');
  } else if (tabName === 'inventory') {
    document.querySelector('.admin-tab:nth-child(2)').classList.add('active');
    document.getElementById('tabInventory').classList.add('active');
  } else if (tabName === 'orders') {
    document.querySelector('.admin-tab:nth-child(3)').classList.add('active');
    document.getElementById('tabOrders').classList.add('active');
    loadAdminOrders();
  } else if (tabName === 'users') {
    document.querySelector('.admin-tab:nth-child(4)').classList.add('active');
    document.getElementById('tabUsers').classList.add('active');
    loadAdminUsers();
  } else if (tabName === 'addProduct') {
    document.querySelector('.admin-tab:nth-child(5)').classList.add('active');
    document.getElementById('tabAddProduct').classList.add('active');
  }
};

async function updateAdminAnalytics() {
  try {
    const res = await fetch('/api/v1/admin/analytics');
    if (res.ok) {
      const json = await res.json();
      const data = json.data || {};

      admTotalRev.textContent = formatVND(data.total_revenue || 0);
      admTotalOrders.textContent = data.total_orders || 0;
      admTotalProds.textContent = state.products.length;
      admLowStockCount.textContent = `${data.low_stock_count || 0} sản phẩm`;

      // Inventory Table
      inventoryTableBody.innerHTML = '';
      state.products.forEach(p => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
          <td>#${p.id}</td>
          <td><strong>${p.name}</strong></td>
          <td>${p.origin}</td>
          <td>${formatVND(p.price)} / ${p.unit}</td>
          <td style="color: ${p.stock < 100 ? 'var(--color-danger)' : 'var(--color-primary-light)'}; font-weight: 700;">${p.stock} ${p.unit}</td>
          <td>
            <button class="btn btn-secondary btn-sm" onclick="restockProduct(${p.id})">+ 100 ${p.unit}</button>
          </td>
        `;
        inventoryTableBody.appendChild(tr);
      });
    }
  } catch (err) {
    console.error('Lỗi nạp admin analytics', err);
  }
}

async function loadAdminOrders() {
  try {
    const res = await fetch('/api/v1/orders/track');
    if (res.ok) {
      const json = await res.json();
      const orders = json.data || [];
      ordersTableBody.innerHTML = '';
      if (orders.length === 0) {
        ordersTableBody.innerHTML = '<tr><td colspan="5" style="text-align: center; color: var(--text-muted);">Chưa có đơn hàng nào trong CSDL.</td></tr>';
        return;
      }
      orders.forEach(o => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
          <td><strong>#${o.id}</strong></td>
          <td>${o.customer_name}</td>
          <td>${o.phone}</td>
          <td style="color: var(--color-primary-light); font-weight: 700;">${formatVND(o.total)}</td>
          <td><span class="cert-badge">${o.status}</span></td>
        `;
        ordersTableBody.appendChild(tr);
      });
    }
  } catch (err) {
    console.error('Lỗi tải danh sách đơn hàng', err);
  }
}

async function loadAdminUsers() {
  try {
    const res = await fetch('/api/v1/admin/users');
    if (res.ok) {
      const json = await res.json();
      const users = json.data || [];
      usersTableBody.innerHTML = '';
      users.forEach(u => {
        const tr = document.createElement('tr');
        const roleClass = u.role === 'ADMIN' ? 'cert-ocop' : u.role === 'FARMER' ? 'cert-vietgap' : 'cert-organic';
        tr.innerHTML = `
          <td>#${u.id}</td>
          <td><strong>${u.name}</strong></td>
          <td>${u.email}</td>
          <td>${u.phone || 'Chưa cập nhật'}</td>
          <td><span class="cert-badge ${roleClass}">${u.role}</span></td>
          <td>${new Date(u.created_at * 1000).toLocaleDateString('vi-VN')}</td>
        `;
        usersTableBody.appendChild(tr);
      });
    }
  } catch (err) {
    console.error('Lỗi tải danh sách người dùng', err);
  }
}

window.restockProduct = async function(productId) {
  const prod = state.products.find(p => p.id === productId);
  if (!prod) return;

  try {
    const res = await fetch('/api/v1/admin/restock', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ product_id: productId, amount: 100 })
    });

    if (res.ok) {
      prod.stock += 100;
      updateAdminAnalytics();
      renderProducts();
      showToast(`Đã nhập thêm 100 ${prod.unit} vào CSDL cho [${prod.name}]!`);
    }
  } catch (err) {
    showToast('Lỗi cập nhật CSDL: ' + err.message, 'error');
  }
};

// Form Add Product to SQLite / MySQL
if (addProductForm) {
  addProductForm.addEventListener('submit', async (e) => {
    e.preventDefault();

    const name = document.getElementById('newProdName').value.trim();
    const category_id = parseInt(document.getElementById('newProdCategory').value);
    const coop_id = parseInt(document.getElementById('newProdCoop').value);
    const price = parseInt(document.getElementById('newProdPrice').value);
    const original_price = parseInt(document.getElementById('newProdOrigPrice').value);
    const unit = document.getElementById('newProdUnit').value.trim();
    const stock = parseInt(document.getElementById('newProdStock').value);
    const region = document.getElementById('newProdRegion').value;
    const cert = document.getElementById('newProdCert').value;
    const description = document.getElementById('newProdDesc').value.trim();
    const origin = document.getElementById('newProdRegion').selectedOptions[0].text;

    const payload = {
      name,
      category_id,
      coop_id,
      price,
      original_price,
      unit,
      stock,
      origin,
      region,
      cert,
      description,
      harvest_date: "Tháng " + (new Date().getMonth() + 1) + "/2026"
    };

    showToast('Đang thêm nông sản mới vào CSDL...', 'info');

    try {
      const res = await fetch('/api/v1/admin/products', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });

      const result = await res.json();
      if (!res.ok || result.status_code >= 400) {
        showToast(result.error || 'Thêm nông sản thất bại!', 'error');
        return;
      }

      addProductForm.reset();
      showToast(`Đã thêm thành công nông sản [${name}] vào CSDL!`);
      await fetchInitialData();
      switchAdminTab('inventory');

    } catch (err) {
      showToast('Lỗi kết nối: ' + err.message, 'error');
    }
  });
}

// ========================================================================
// 8. Real-Time RFC 6455 WebSocket Client & Live Stream
// ========================================================================
function initWebSocket() {
  const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
  const wsUrl = `${protocol}//${location.host}/ws`;
  
  let ws;
  try {
    ws = new WebSocket(wsUrl);
  } catch (e) {
    console.warn('[WebSocket Init Error]', e);
    return;
  }

  const wsBadge = document.getElementById('wsLiveStatus');

  ws.onopen = () => {
    console.log('[VietLang WebSocket] Connection Established to', wsUrl);
    if (wsBadge) {
      wsBadge.innerHTML = `<span style="width:7px;height:7px;border-radius:50%;background:#10B981;display:inline-block;"></span> WEBSOCKET LIVE`;
      wsBadge.style.color = '#10B981';
    }
  };

  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      console.log('[VietLang WS Event]', data);

      if (data.type === 'order_placed') {
        showToast(`⚡ [LIVE WS] Khách hàng ${data.customer_name} (${data.city || 'Việt Nam'}) vừa đặt đơn ${formatVND(data.total)}!`, 'success');
        fetchInitialData();
      } else if (data.type === 'stock_updated') {
        showToast(`📦 [LIVE WS] Kho vừa nhập thêm +${data.amount} sản phẩm!`, 'info');
        fetchInitialData();
      } else if (data.type === 'product_created') {
        showToast(`🌿 [LIVE WS] Nông sản mới [${data.product_name}] vừa lên kệ!`, 'success');
        fetchInitialData();
      } else if (data.type === 'user_registered') {
        showToast(`👤 [LIVE WS] Thành viên mới [${data.user_name}] (${data.role}) vừa gia nhập!`, 'info');
      }
    } catch (e) {
      console.log('[WS Raw Message]', event.data);
    }
  };

  ws.onclose = () => {
    if (wsBadge) {
      wsBadge.innerHTML = `<span style="width:7px;height:7px;border-radius:50%;background:#F59E0B;display:inline-block;"></span> WS RECONNECTING`;
      wsBadge.style.color = '#F59E0B';
    }
    setTimeout(initWebSocket, 3000);
  };

  ws.onerror = (err) => {
    console.warn('[WS Error]', err);
  };
}

initFlashSaleTimer();
fetchInitialData();
initWebSocket();

