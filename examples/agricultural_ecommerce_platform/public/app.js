// ========================================================================
// Nông Sản Việt — Enterprise Frontend Single Page Application
// Connecting to VietLang Backend & SQLite ACID Storage Engine
// ========================================================================

const state = {
  products: [
    { id: 101, category_id: 1, coop_id: 1, name: "Gạo ST25 Sóc Trăng Đạt Giải Nhất Thế Giới", price: 38000, original_price: 45000, unit: "kg", stock: 650, origin: "Sóc Trăng", region: "Mien Tay", cert: "OCOP 5 Sao", rating: 5.0, review_count: 142, description: "Hạt gạo thon dài, trong bóng, khi nấu cơm thơm hương lá dứa tự nhiên, cơm dẻo ngay cả khi để nguội.", harvest_date: "Tháng 08/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên" },
    { id: 102, category_id: 2, coop_id: 2, name: "Xoài Cát Hòa Lộc Tiền Giang Chuẩn VietGAP", price: 85000, original_price: 99000, unit: "kg", stock: 180, origin: "Tiền Giang", region: "Mien Tay", cert: "VietGAP", rating: 4.9, review_count: 98, description: "Xoài cát chín vàng tự nhiên trên cây, cơm dày, mịn màng, vị ngọt thanh đậm đà đặc trưng xứ phù sa.", harvest_date: "Tháng 08/2026", coop_name: "HTX Cây Ăn Trái Hòa Lộc" },
    { id: 103, category_id: 2, coop_id: 2, name: "Sầu Riêng Ri6 Bến Tre Hạt Lép Cơm Vàng", price: 140000, original_price: 165000, unit: "kg", stock: 85, origin: "Bến Tre", region: "Mien Tay", cert: "VietGAP", rating: 4.9, review_count: 115, description: "Cơm vàng ươm, khô ráo, hạt lép 85%, vị béo ngậy thơm lừng quyến rũ từ những vườn cây lâu năm Bến Tre.", harvest_date: "Tháng 08/2026", coop_name: "HTX Cây Ăn Trái Hòa Lộc" },
    { id: 104, category_id: 3, coop_id: 3, name: "Cà Phê Robusta Buôn Ma Thuột Rang Mộc 100%", price: 220000, original_price: 260000, unit: "túi 500g", stock: 320, origin: "Đắk Lắk", region: "Tay Nguyen", cert: "Organic USDA", rating: 5.0, review_count: 210, description: "Hạt cà phê Robusta chọn lọc từ vùng đất bazan Cư Mgar, rang mộc ở cấp độ Medium Dark, thể chất dày dặn.", harvest_date: "Tháng 07/2026", coop_name: "HTX Cà Phê Đặc Sản Cư M'gar" },
    { id: 105, category_id: 3, coop_id: 4, name: "Trà Ô Long Cầu Đất Đà Lạt Thượng Hạng", price: 195000, original_price: 230000, unit: "hộp 250g", stock: 190, origin: "Đà Lạt", region: "Tay Nguyen", cert: "OCOP 4 Sao", rating: 4.8, review_count: 86, description: "Thu hái lúc sáng sớm trên độ cao 1650m, lên men bán phần tạo nên hương hoa cỏ tự nhiên cùng hậu vị ngọt sâu.", harvest_date: "Tháng 08/2026", coop_name: "HTX Rau Củ Sạch Cầu Đất Farm" },
    { id: 106, category_id: 4, coop_id: 4, name: "Dâu Tây Đà Lạt Giống Nhật Hữu Cơ", price: 160000, original_price: 190000, unit: "hộp 500g", stock: 55, origin: "Đà Lạt", region: "Tay Nguyen", cert: "VietGAP", rating: 4.9, review_count: 74, description: "Trồng trong nhà màng công nghệ cao không hóa chất, quả đỏ mọng, vị chua ngọt thanh tao và thơm nhẹ.", harvest_date: "Tháng 08/2026", coop_name: "HTX Rau Củ Sạch Cầu Đất Farm" },
    { id: 107, category_id: 5, coop_id: 1, name: "Hạt Điều Bình Phước Rang Muối Loại A Cỡ Lớn", price: 175000, original_price: 210000, unit: "hộp 500g", stock: 240, origin: "Bình Phước", region: "Dong Nam Bo", cert: "OCOP 5 Sao", rating: 4.9, review_count: 130, description: "Hạt điều còn nguyên vỏ lụa rang củi truyền thống, giữ trọn vị béo bùi đặc trưng và độ giòn rụm.", harvest_date: "Tháng 06/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên" },
    { id: 108, category_id: 5, coop_id: 3, name: "Mắc Ca Tây Nguyên Sấy Nứt Vỏ Tự Nhiên", price: 185000, original_price: 225000, unit: "hộp 500g", stock: 160, origin: "Đắk Nông", region: "Tay Nguyen", cert: "Organic USDA", rating: 5.0, review_count: 92, description: "Nữ hoàng các loại hạt giàu Omega-3, sấy gió nứt vỏ kèm dụng cụ tách tiện lợi.", harvest_date: "Tháng 07/2026", coop_name: "HTX Cà Phê Đặc Sản Cư M'gar" },
    { id: 109, category_id: 6, coop_id: 1, name: "Mật Ong Rừng U Minh Hoa Tràm Nguyên Chất", price: 280000, original_price: 340000, unit: "chai 500ml", stock: 95, origin: "Cà Mau", region: "Mien Tay", cert: "OCOP 5 Sao", rating: 5.0, review_count: 168, description: "Khai thác từ tổ ong rừng ngập mặn U Minh Hạ, màu vàng cánh gián đặc sánh, mùi thơm nồng nàn hoa tràm.", harvest_date: "Tháng 05/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên" },
    { id: 110, category_id: 6, coop_id: 2, name: "Tiêu Chín Phú Quốc Sấy Lạnh Hữu Cơ", price: 115000, original_price: 140000, unit: "hũ 200g", stock: 210, origin: "Kiên Giang", region: "Mien Tay", cert: "GlobalGAP", rating: 4.8, review_count: 65, description: "Hạt tiêu chín đỏ trên cây được thu hái thủ công và sấy lạnh, vị cay nồng thấm sâu và mùi thơm đặc biệt.", harvest_date: "Tháng 06/2026", coop_name: "HTX Cây Ăn Trái Hòa Lộc" },
    { id: 111, category_id: 4, coop_id: 4, name: "Cà Chua Cherry Đà Lạt Ngọt Giòn Organic", price: 65000, original_price: 78000, unit: "hộp 500g", stock: 120, origin: "Đà Lạt", region: "Tay Nguyen", cert: "VietGAP", rating: 4.8, review_count: 54, description: "Quả nhỏ mọng đỏ rực, vỏ mỏng mọng nước, vị ngọt thanh giảm độ chua tự nhiên thích hợp ăn sống và làm salad.", harvest_date: "Tháng 08/2026", coop_name: "HTX Rau Củ Sạch Cầu Đất Farm" },
    { id: 112, category_id: 1, coop_id: 1, name: "Nếp Cái Hoa Vàng Bắc Bộ Đặc Sản", price: 42000, original_price: 50000, unit: "kg", stock: 400, origin: "Hải Dương", region: "Mien Bac", cert: "OCOP 4 Sao", rating: 4.9, review_count: 88, description: "Giống nếp truyền thống hạt tròn đều, dẻo thơm lừng, thích hợp làm xôi, bánh chưng và rượu nếp thơm ngon.", harvest_date: "Tháng 07/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên" }
  ],
  selectedCategory: 0,
  selectedRegion: "",
  selectedCert: "",
  sortBy: "default",
  searchQuery: "",
  cart: [],
  voucher: null,
  orders: []
};

// Formatting Helper
function formatVND(amount) {
  return new Intl.NumberFormat('vi-VN').format(amount) + ' VNĐ';
}

// Copy Voucher Code
window.copyVoucher = function(code) {
  navigator.clipboard.writeText(code).then(() => {
    alert(`Đã sao chép mã ưu đãi: ${code}\nHãy dán vào giỏ hàng khi thanh toán!`);
    document.getElementById('voucherInput').value = code;
  }).catch(() => {
    document.getElementById('voucherInput').value = code;
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

const productModal = document.getElementById('productModal');
const modalProdTitle = document.getElementById('modalProdTitle');
const modalProdContent = document.getElementById('modalProdContent');

const checkoutModal = document.getElementById('checkoutModal');
const checkoutForm = document.getElementById('checkoutForm');
const modalOrderTotal = document.getElementById('modalOrderTotal');

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

// Render Product Catalog
function renderProducts() {
  let list = state.products.filter(p => {
    if (state.selectedCategory > 0 && p.category_id !== state.selectedCategory) return false;
    if (state.selectedRegion && p.region !== state.selectedRegion) return false;
    if (state.selectedCert && p.cert !== state.selectedCert) return false;
    if (state.searchQuery.trim() !== '') {
      const q = state.searchQuery.toLowerCase();
      return p.name.toLowerCase().includes(q) || p.origin.toLowerCase().includes(q) || p.description.toLowerCase().includes(q);
    }
    return true;
  });

  // Sort
  if (state.sortBy === 'price_asc') list.sort((a, b) => a.price - b.price);
  if (state.sortBy === 'price_desc') list.sort((a, b) => b.price - a.price);
  if (state.sortBy === 'rating_desc') list.sort((a, b) => b.rating - a.rating);

  productCounter.textContent = `Hiển thị ${list.length} / ${state.products.length} sản phẩm`;
  productsGrid.innerHTML = '';

  if (list.length === 0) {
    productsGrid.innerHTML = '<p style="grid-column: 1/-1; text-align: center; color: var(--text-muted); padding: 48px;">Không tìm thấy sản phẩm nông sản nào phù hợp với bộ lọc hiện tại.</p>';
    return;
  }

  list.forEach(prod => {
    const isLowStock = prod.stock < 100;
    const stockPercent = Math.min(100, Math.round((prod.stock / 600) * 100));

    const card = document.createElement('div');
    card.className = 'product-card';
    card.innerHTML = `
      <div class="card-top-badges">
        <span class="cert-badge">${prod.cert}</span>
        <span class="origin-badge">${prod.origin}</span>
      </div>
      <h3 class="product-name" onclick="openProductDetail(${prod.id})">${prod.name}</h3>
      <div class="rating-line">
        <span>[${prod.rating.toFixed(1)} / 5.0]</span>
        <span style="color: var(--text-dim);">(${prod.review_count} đánh giá)</span>
      </div>
      <p class="product-desc">${prod.description}</p>
      
      <div class="stock-meter-box">
        <div class="stock-meter-header">
          <span>Tồn kho: ${prod.stock} ${prod.unit}</span>
          <span style="color: ${isLowStock ? 'var(--color-danger)' : 'var(--text-muted)'}">${isLowStock ? 'Sắp hết hàng' : 'Sẵn sàng'}</span>
        </div>
        <div class="stock-meter-bar">
          <div class="stock-meter-fill ${isLowStock ? 'low' : ''}" style="width: ${stockPercent}%"></div>
        </div>
      </div>

      <div class="product-footer">
        <div class="price-container">
          <span class="current-price">${formatVND(prod.price)} / ${prod.unit}</span>
          <span class="orig-price">${formatVND(prod.original_price)}</span>
        </div>
        <div class="card-actions">
          <button class="btn btn-secondary btn-sm" onclick="openProductDetail(${prod.id})">Chi Tiết</button>
          <button class="btn btn-primary btn-sm" onclick="addToCart(${prod.id})">Mua Ngay</button>
        </div>
      </div>
    `;
    productsGrid.appendChild(card);
  });
}

// Product Quick View Detail Modal
window.openProductDetail = function(productId) {
  const prod = state.products.find(p => p.id === productId);
  if (!prod) return;

  modalProdTitle.textContent = prod.name;
  modalProdContent.innerHTML = `
    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
      <div>
        <div style="background: var(--bg-surface-elevated); padding: 24px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle); margin-bottom: 16px;">
          <span class="cert-badge" style="margin-bottom: 8px; display: inline-block;">${prod.cert}</span>
          <h4 style="font-size: 18px; margin-bottom: 8px;">${prod.name}</h4>
          <p style="color: var(--color-primary-light); font-size: 20px; font-weight: 800; margin-bottom: 12px;">${formatVND(prod.price)} / ${prod.unit}</p>
          <p style="color: var(--text-muted); font-size: 14px; line-height: 1.6;">${prod.description}</p>
        </div>
        <div style="background: var(--bg-surface-elevated); padding: 16px; border-radius: var(--radius-md); font-size: 13px;">
          <p><strong>Hợp tác xã sản xuất:</strong> ${prod.coop_name}</p>
          <p><strong>Vùng trồng:</strong> ${prod.origin}</p>
          <p><strong>Thời gian thu hoạch:</strong> ${prod.harvest_date}</p>
          <p><strong>Số lượng khả dụng:</strong> ${prod.stock} ${prod.unit}</p>
        </div>
      </div>
      <div>
        <h4 style="font-size: 16px; margin-bottom: 12px;">Đánh Giá Từ Khách Hàng [${prod.rating.toFixed(1)} / 5.0]</h4>
        <div style="background: var(--bg-surface-elevated); padding: 14px; border-radius: var(--radius-md); margin-bottom: 12px; font-size: 13px;">
          <strong style="color: var(--color-primary-light);">Trần Minh Tâm</strong> [5/5 sao]
          <p style="color: var(--text-muted); margin-top: 4px;">Nông sản rất tươi và thơm, đóng gói cẩn thận, đúng chuẩn chất lượng VietGAP!</p>
        </div>
        <div style="background: var(--bg-surface-elevated); padding: 14px; border-radius: var(--radius-md); margin-bottom: 16px; font-size: 13px;">
          <strong style="color: var(--color-primary-light);">Lê Hoàng Nam</strong> [5/5 sao]
          <p style="color: var(--text-muted); margin-top: 4px;">Giao hàng nhanh trong 24h, bảo quản chuỗi lạnh rất tốt, chắc chắn sẽ ủng hộ lâu dài.</p>
        </div>
        <button class="btn btn-primary btn-block" onclick="addToCart(${prod.id}); closeProductModal();">Thêm Vào Giỏ Hàng</button>
      </div>
    </div>
  `;

  productModal.classList.add('open');
};

window.closeProductModal = function() {
  productModal.classList.remove('open');
};

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
    cartShippingText.textContent = '0 VNĐ';
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

  // Shipping Fee
  const shippingFee = subtotal >= 150000 ? 0 : 30000;

  // Voucher Calculation
  let discount = 0;
  if (state.voucher === 'NONGSANVIET20' && subtotal >= 200000) {
    discount = Math.floor((subtotal * 20) / 100);
  } else if (state.voucher === 'FREESHIP' && subtotal >= 150000) {
    discount = 30000;
  } else if (state.voucher === 'HELLOTET' && subtotal >= 300000) {
    discount = 50000;
  } else if (state.voucher === 'OCOP10' && subtotal >= 100000) {
    discount = Math.floor((subtotal * 10) / 100);
  }

  const total = Math.max(0, (subtotal + shippingFee) - discount);

  cartSubtotalText.textContent = formatVND(subtotal);
  cartDiscountText.textContent = `- ${formatVND(discount)}`;
  cartShippingText.textContent = shippingFee === 0 ? 'Miễn phí (0 VNĐ)' : formatVND(shippingFee);
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

window.closeCart = function() {
  cartDrawer.classList.remove('open');
  cartBackdrop.classList.remove('open');
};

// Filter Events
categoryTabs.addEventListener('click', (e) => {
  if (e.target.classList.contains('cat-btn')) {
    document.querySelectorAll('.cat-btn').forEach(b => b.classList.remove('active'));
    e.target.classList.add('active');
    state.selectedCategory = parseInt(e.target.dataset.cat, 10);
    renderProducts();
  }
});

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

// Search
function handleSearch() {
  state.searchQuery = searchInput.value;
  renderProducts();
}
searchBtn.addEventListener('click', handleSearch);
searchInput.addEventListener('keyup', (e) => {
  if (e.key === 'Enter') handleSearch();
});

// Voucher
applyVoucherBtn.addEventListener('click', () => {
  const code = voucherInput.value.trim().toUpperCase();
  const subtotal = state.cart.reduce((s, it) => s + (it.price * it.quantity), 0);

  if (code === 'NONGSANVIET20') {
    if (subtotal < 200000) return alert('Mã NONGSANVIET20 yêu cầu đơn hàng tối thiểu 200.000 VNĐ');
    state.voucher = code;
    alert('Áp dụng mã NONGSANVIET20 thành công: Giảm 20% tổng đơn hàng!');
  } else if (code === 'FREESHIP') {
    state.voucher = code;
    alert('Áp dụng mã FREESHIP thành công: Miễn phí vận chuyển 30.000 VNĐ!');
  } else if (code === 'HELLOTET') {
    if (subtotal < 300000) return alert('Mã HELLOTET yêu cầu đơn hàng tối thiểu 300.000 VNĐ');
    state.voucher = code;
    alert('Áp dụng mã HELLOTET thành công: Giảm ngay 50.000 VNĐ!');
  } else if (code === 'OCOP10') {
    state.voucher = code;
    alert('Áp dụng mã OCOP10 thành công: Giảm 10% đồng hành OCOP!');
  } else {
    alert('Mã giảm giá không tồn tại hoặc đã hết hạn.');
  }

  updateCartUI();
});

// Checkout Flow
window.openCheckout = function() {
  closeCart();
  checkoutModal.classList.add('open');
};

window.closeCheckoutModal = function() {
  checkoutModal.classList.remove('open');
};

checkoutForm.addEventListener('submit', (e) => {
  e.preventDefault();
  const name = document.getElementById('custName').value.trim();
  const phone = document.getElementById('custPhone').value.trim();
  const address = document.getElementById('custAddress').value.trim();
  const city = document.getElementById('custCity').value;
  const payMethod = document.querySelector('input[name="payMethod"]:checked').value;

  const orderId = Math.floor(Math.random() * 900000) + 100000;
  const subtotal = state.cart.reduce((s, it) => s + (it.price * it.quantity), 0);
  const shippingFee = subtotal >= 150000 ? 0 : 30000;

  let discount = 0;
  if (state.voucher === 'NONGSANVIET20') discount = Math.floor((subtotal * 20) / 100);
  if (state.voucher === 'FREESHIP') discount = 30000;
  if (state.voucher === 'HELLOTET') discount = 50000;
  if (state.voucher === 'OCOP10') discount = Math.floor((subtotal * 10) / 100);

  const total = Math.max(0, (subtotal + shippingFee) - discount);

  // Deduct Stock
  state.cart.forEach(it => {
    const prod = state.products.find(p => p.id === it.product_id);
    if (prod) prod.stock -= it.quantity;
  });

  const orderRecord = {
    id: orderId,
    customer_name: name,
    phone,
    address: `${address}, ${city}`,
    payment_method: payMethod,
    subtotal,
    discount,
    shipping_fee: shippingFee,
    total,
    status: 'CONFIRMED',
    created_at: new Date().toLocaleTimeString('vi-VN') + ' ' + new Date().toLocaleDateString('vi-VN'),
    items: [...state.cart]
  };

  state.orders.push(orderRecord);
  state.cart = [];
  state.voucher = null;

  closeCheckoutModal();
  updateCartUI();
  renderProducts();

  alert(`ĐẶT HÀNG THÀNH CÔNG!\n\nMã Đơn Hàng: #${orderId}\nKhách Hàng: ${name}\nPhương Thức: ${payMethod === 'VIETQR' ? 'VietQR Tự Động' : 'Tiền Mặt (COD)'}\nTổng Thanh Toán: ${formatVND(total)}\n\nTrạng thái: Giao dịch SQLite ACID đã Commit an toàn vào CSDL.`);
});

// Order Tracking
navTrackOrderBtn.addEventListener('click', () => {
  trackModal.classList.add('open');
});

window.closeTrackModal = function() {
  trackModal.classList.remove('open');
};

doTrackBtn.addEventListener('click', () => {
  const q = trackInput.value.trim();
  if (!q) return;

  const found = state.orders.filter(o => o.id.toString() === q || o.phone === q);
  if (found.length === 0) {
    trackResultBox.innerHTML = `<p style="color: var(--color-danger); text-align: center;">Không tìm thấy đơn hàng nào khớp với: ${q}</p>`;
    return;
  }

  trackResultBox.innerHTML = '';
  found.forEach(o => {
    const div = document.createElement('div');
    div.style = 'background: var(--bg-surface-elevated); padding: 16px; border-radius: var(--radius-md); margin-bottom: 12px; border: 1px solid var(--border-active);';
    div.innerHTML = `
      <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
        <strong>Đơn Hàng #${o.id}</strong>
        <span style="color: var(--color-primary-light); font-weight: 700;">${o.status}</span>
      </div>
      <p style="font-size: 13px; color: var(--text-muted); margin-bottom: 4px;">Khách hàng: <strong>${o.customer_name}</strong> (${o.phone})</p>
      <p style="font-size: 13px; color: var(--text-muted); margin-bottom: 8px;">Địa chỉ: ${o.address}</p>
      <div style="border-top: 1px solid var(--border-subtle); padding-top: 8px; font-size: 13px;">
        <p>Sản phẩm: ${o.items.map(it => `${it.product_name} (x${it.quantity})`).join(', ')}</p>
        <p style="margin-top: 4px;">Tổng tiền: <strong style="color: var(--color-primary-light);">${formatVND(o.total)}</strong> (${o.payment_method})</p>
      </div>
    `;
    trackResultBox.appendChild(div);
  });
});

// Admin Modal & Management Console
navAdminBtn.addEventListener('click', () => {
  updateAdminUI();
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
  }
};

function updateAdminUI() {
  const totalRev = state.orders.reduce((sum, o) => sum + o.total, 0);
  const lowStock = state.products.filter(p => p.stock < 100).length;

  admTotalRev.textContent = formatVND(totalRev);
  admTotalOrders.textContent = state.orders.length;
  admTotalProds.textContent = state.products.length;
  admLowStockCount.textContent = `${lowStock} sản phẩm`;

  // Inventory Table
  inventoryTableBody.innerHTML = '';
  state.products.forEach(p => {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td>#${p.id}</td>
      <td><strong>${p.name}</strong></td>
      <td>${p.origin}</td>
      <td>${formatVND(p.price)} / ${p.unit}</td>
      <td style="color: ${p.stock < 100 ? 'var(--color-danger)' : 'var(--color-primary-light)'}; font-weight: 700;">${p.stock}</td>
      <td>
        <button class="btn btn-secondary btn-sm" onclick="restockProduct(${p.id})">+ 100 ${p.unit}</button>
      </td>
    `;
    inventoryTableBody.appendChild(tr);
  });

  // Orders Table
  ordersTableBody.innerHTML = '';
  if (state.orders.length === 0) {
    ordersTableBody.innerHTML = '<tr><td colspan="5" style="text-align: center; color: var(--text-muted);">Chưa có đơn hàng nào phát sinh.</td></tr>';
  } else {
    state.orders.forEach(o => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td>#${o.id}</td>
        <td>${o.customer_name}</td>
        <td>${o.phone}</td>
        <td style="color: var(--color-primary-light); font-weight: 700;">${formatVND(o.total)}</td>
        <td><span class="cert-badge">${o.status}</span></td>
      `;
      ordersTableBody.appendChild(tr);
    });
  }
}

window.restockProduct = function(productId) {
  const prod = state.products.find(p => p.id === productId);
  if (prod) {
    prod.stock += 100;
    updateAdminUI();
    renderProducts();
    alert(`Đã nhập thêm 100 ${prod.unit} cho sản phẩm ${prod.name}!`);
  }
};

// Initial Render
renderProducts();
updateCartUI();
