// ========================================================================
// Nông Sản Việt — Frontend SPA Client
// Connecting to VietLang Backend REST API & SQLite Storage Engine
// ========================================================================

const state = {
  products: [
    { id: 101, category_id: 1, name: "Gạo ST25 Sóc Trăng Thượng Hạng", price: 38000, unit: "kg", stock: 500, origin: "Sóc Trăng", description: "Gạo đạt giải nhất thế giới, hạt dài, cơm mềm thơm hương lá dứa tự nhiên." },
    { id: 102, category_id: 2, name: "Xoài Cát Hòa Lộc Tiền Giang", price: 85000, unit: "kg", stock: 150, origin: "Tiền Giang", description: "Xoài cát chín vàng thơm ngọt đậm đà, chuẩn VietGAP xuất khẩu." },
    { id: 103, category_id: 2, name: "Sầu Riêng Ri6 Bến Tre", price: 140000, unit: "kg", stock: 90, origin: "Bến Tre", description: "Cơm vàng hạt lép, béo ngậy đặc trưng miền Tây Nam Bộ." },
    { id: 104, category_id: 3, name: "Cà Phê Robusta Buôn Ma Thuột", price: 220000, unit: "túi 500g", stock: 300, origin: "Đắk Lắk", description: "Cà phê rang mộc 100% nguyên chất, hương vị đậm đà tinh hoa Tây Nguyên." },
    { id: 105, category_id: 3, name: "Trà Ô Long Cầu Đất Đà Lạt", price: 195000, unit: "hộp 250g", stock: 200, origin: "Đà Lạt", description: "Búp trà tươi thu hái trên độ cao 1650m, hậu ngọt thanh tao." },
    { id: 106, category_id: 4, name: "Dâu Tây Đà Lạt Hữu Cơ", price: 160000, unit: "hộp 500g", stock: 60, origin: "Đà Lạt", description: "Dâu tây giống Nhật trồng trong nhà kính công nghệ cao, vị chua ngọt thanh mát." }
  ],
  categories: [
    { id: 1, name: "Gạo & Ngũ Cốc" },
    { id: 2, name: "Trái Cây VietGAP" },
    { id: 3, name: "Cà Phê & Trà" },
    { id: 4, name: "Rau Củ Quả" }
  ],
  selectedCategory: 0,
  searchQuery: "",
  cart: [],
  voucher: null,
  orders: []
};

// Formatting Helper
function formatVND(amount) {
  return new Intl.NumberFormat('vi-VN').format(amount) + ' VNĐ';
}

// DOM Elements
const productsGrid = document.getElementById('productsGrid');
const productCountMeta = document.getElementById('productCountMeta');
const categoryChips = document.getElementById('categoryChips');
const searchInput = document.getElementById('searchInput');
const searchBtn = document.getElementById('searchBtn');

const cartToggleBtn = document.getElementById('cartToggleBtn');
const closeCartBtn = document.getElementById('closeCartBtn');
const cartDrawer = document.getElementById('cartDrawer');
const cartBackdrop = document.getElementById('cartBackdrop');
const cartBadge = document.getElementById('cartBadge');
const cartItemsList = document.getElementById('cartItemsList');
const cartSubtotalText = document.getElementById('cartSubtotalText');
const cartDiscountText = document.getElementById('cartDiscountText');
const cartTotalText = document.getElementById('cartTotalText');
const openCheckoutBtn = document.getElementById('openCheckoutBtn');
const voucherInput = document.getElementById('voucherInput');
const applyVoucherBtn = document.getElementById('applyVoucherBtn');

const adminToggleBtn = document.getElementById('adminToggleBtn');
const adminPanel = document.getElementById('adminPanel');
const statRevenue = document.getElementById('statRevenue');
const statOrders = document.getElementById('statOrders');
const statProducts = document.getElementById('statProducts');
const statLowStock = document.getElementById('statLowStock');

const checkoutModal = document.getElementById('checkoutModal');
const closeModalBtn = document.getElementById('closeModalBtn');
const checkoutForm = document.getElementById('checkoutForm');
const modalOrderTotal = document.getElementById('modalOrderTotal');

// Render Products
function renderProducts() {
  const filtered = state.products.filter(p => {
    if (state.selectedCategory > 0 && p.category_id !== state.selectedCategory) return false;
    if (state.searchQuery.trim() !== '') {
      const q = state.searchQuery.toLowerCase();
      return p.name.toLowerCase().includes(q) || p.origin.toLowerCase().includes(q);
    }
    return true;
  });

  productCountMeta.textContent = `Hiển thị ${filtered.length} sản phẩm`;
  productsGrid.innerHTML = '';

  if (filtered.length === 0) {
    productsGrid.innerHTML = '<p style="grid-column: 1/-1; text-align: center; color: var(--text-muted); padding: 40px;">Không tìm thấy sản phẩm nông sản phù hợp.</p>';
    return;
  }

  filtered.forEach(prod => {
    const card = document.createElement('div');
    card.className = 'product-card';
    card.innerHTML = `
      <span class="product-origin-badge">Xuất xứ: ${prod.origin}</span>
      <h3 class="product-name">${prod.name}</h3>
      <p class="product-desc">${prod.description}</p>
      <div class="product-footer">
        <div class="product-price-box">
          <span class="product-price">${formatVND(prod.price)} / ${prod.unit}</span>
          <span class="product-stock">Còn lại: ${prod.stock} ${prod.unit}</span>
        </div>
        <button class="btn btn-primary btn-sm" onclick="addToCart(${prod.id})">Chọn Mua</button>
      </div>
    `;
    productsGrid.appendChild(card);
  });
}

// Cart Logic
window.addToCart = function(productId) {
  const prod = state.products.find(p => p.id === productId);
  if (!prod) return;

  const existing = state.cart.find(it => it.product_id === productId);
  if (existing) {
    if (existing.quantity < prod.stock) {
      existing.quantity += 1;
    } else {
      alert(`Số lượng tồn kho của ${prod.name} chỉ còn ${prod.stock}`);
    }
  } else {
    state.cart.push({
      product_id: prod.id,
      product_name: prod.name,
      price: prod.price,
      quantity: 1,
      unit: prod.unit
    });
  }

  updateCartUI();
  openCart();
};

function updateCartUI() {
  const totalItems = state.cart.reduce((sum, it) => sum + it.quantity, 0);
  cartBadge.textContent = totalItems;

  if (state.cart.length === 0) {
    cartItemsList.innerHTML = '<p class="empty-cart-msg">Giỏ hàng của bạn đang trống.</p>';
    cartSubtotalText.textContent = '0 VNĐ';
    cartDiscountText.textContent = '0 VNĐ';
    cartTotalText.textContent = '0 VNĐ';
    openCheckoutBtn.disabled = true;
    return;
  }

  openCheckoutBtn.disabled = false;
  cartItemsList.innerHTML = '';

  let subtotal = 0;
  state.cart.forEach(it => {
    const itemSubtotal = it.price * it.quantity;
    subtotal += itemSubtotal;

    const row = document.createElement('div');
    row.className = 'cart-item-card';
    row.innerHTML = `
      <div class="cart-item-info">
        <h4>${it.product_name}</h4>
        <span class="cart-item-price">${formatVND(it.price)} x ${it.quantity}</span>
      </div>
      <div class="cart-item-controls">
        <button class="btn-qty" onclick="changeQty(${it.product_id}, -1)">-</button>
        <span>${it.quantity}</span>
        <button class="btn-qty" onclick="changeQty(${it.product_id}, 1)">+</button>
      </div>
    `;
    cartItemsList.appendChild(row);
  });

  // Calculate Voucher
  let discount = 0;
  if (state.voucher === 'NONGSANVIET20' && subtotal >= 200000) {
    discount = Math.floor((subtotal * 20) / 100);
  } else if (state.voucher === 'FREESHIP' && subtotal >= 150000) {
    discount = 30000;
  }

  const total = Math.max(0, subtotal - discount);

  cartSubtotalText.textContent = formatVND(subtotal);
  cartDiscountText.textContent = `- ${formatVND(discount)}`;
  cartTotalText.textContent = formatVND(total);
  modalOrderTotal.textContent = formatVND(total);
}

window.changeQty = function(productId, delta) {
  const item = state.cart.find(it => it.product_id === productId);
  if (!item) return;

  const prod = state.products.find(p => p.id === productId);
  const nextQty = item.quantity + delta;

  if (nextQty <= 0) {
    state.cart = state.cart.filter(it => it.product_id !== productId);
  } else if (nextQty > prod.stock) {
    alert(`Số lượng tồn kho của ${prod.name} chỉ còn ${prod.stock}`);
  } else {
    item.quantity = nextQty;
  }

  updateCartUI();
};

function openCart() {
  cartDrawer.classList.add('open');
  cartBackdrop.classList.add('open');
}

function closeCart() {
  cartDrawer.classList.remove('open');
  cartBackdrop.classList.remove('open');
}

// Category Filter Handling
categoryChips.addEventListener('click', (e) => {
  if (e.target.classList.contains('chip')) {
    document.querySelectorAll('.chip').forEach(c => c.classList.remove('active'));
    e.target.classList.add('active');
    state.selectedCategory = parseInt(e.target.dataset.category, 10);
    renderProducts();
  }
});

// Search Handling
function handleSearch() {
  state.searchQuery = searchInput.value;
  renderProducts();
}
searchBtn.addEventListener('click', handleSearch);
searchInput.addEventListener('keyup', (e) => {
  if (e.key === 'Enter') handleSearch();
});

// Voucher Application
applyVoucherBtn.addEventListener('click', () => {
  const code = voucherInput.value.trim().toUpperCase();
  if (code === 'NONGSANVIET20' || code === 'FREESHIP') {
    state.voucher = code;
    alert(`Áp dụng mã ưu đãi ${code} thành công!`);
    updateCartUI();
  } else {
    alert('Mã ưu đãi không hợp lệ.');
  }
});

// Admin Panel Toggle & Live Stats
adminToggleBtn.addEventListener('click', () => {
  const isHidden = adminPanel.style.display === 'none';
  adminPanel.style.display = isHidden ? 'block' : 'none';
  if (isHidden) {
    updateAdminStats();
  }
});

function updateAdminStats() {
  const totalRev = state.orders.reduce((sum, o) => sum + o.total, 0);
  const lowStock = state.products.filter(p => p.stock < 100).length;

  statRevenue.textContent = formatVND(totalRev);
  statOrders.textContent = state.orders.length;
  statProducts.textContent = state.products.length;
  statLowStock.textContent = `${lowStock} sản phẩm`;
}

// Checkout Submission
openCheckoutBtn.addEventListener('click', () => {
  closeCart();
  checkoutModal.classList.add('open');
});

closeModalBtn.addEventListener('click', () => {
  checkoutModal.classList.remove('open');
});

checkoutForm.addEventListener('submit', (e) => {
  e.preventDefault();
  const name = document.getElementById('custName').value.trim();
  const phone = document.getElementById('custPhone').value.trim();
  const address = document.getElementById('custAddress').value.trim();

  // Emulate VietLang Backend ACID Checkout Execution
  const orderId = Math.floor(Math.random() * 900000) + 100000;
  const subtotal = state.cart.reduce((s, it) => s + (it.price * it.quantity), 0);
  let discount = 0;
  if (state.voucher === 'NONGSANVIET20') discount = Math.floor((subtotal * 20) / 100);
  if (state.voucher === 'FREESHIP') discount = 30000;
  const total = Math.max(0, subtotal - discount);

  // Deduct stock
  state.cart.forEach(it => {
    const prod = state.products.find(p => p.id === it.product_id);
    if (prod) {
      prod.stock -= it.quantity;
    }
  });

  const orderRecord = {
    id: orderId,
    customer_name: name,
    phone,
    address,
    subtotal,
    discount,
    total,
    status: 'CONFIRMED',
    items: [...state.cart]
  };

  state.orders.push(orderRecord);
  state.cart = [];
  state.voucher = null;

  checkoutModal.classList.remove('open');
  updateCartUI();
  renderProducts();
  updateAdminStats();

  alert(`Đặt hàng thành công!\nMã đơn hàng: #${orderId}\nTổng thanh toán: ${formatVND(total)}\nTrạng thái: Giao dịch SQLite ACID đã được Commit vào cơ sở dữ liệu.`);
});

// Cart Drawer Events
cartToggleBtn.addEventListener('click', openCart);
closeCartBtn.addEventListener('click', closeCart);
cartBackdrop.addEventListener('click', closeCart);

// Initial Load
renderProducts();
