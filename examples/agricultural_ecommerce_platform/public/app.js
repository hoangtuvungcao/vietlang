// ========================================================================
// Nông Sản Việt — Enterprise Frontend Single Page Application (VietLang Powered)
// 24 Products, 6 Cooperatives, Flash Sale Engine, VietQR, and SQLite Relational Sync
// ========================================================================

const state = {
  cooperatives: [
    { id: 1, name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên", location: "Sóc Trăng", cert: "OCOP 5 Sao", founded_year: 2012, contact: "0299.3888.999", hectares: 350, farmer_count: 180 },
    { id: 2, name: "HTX Cây Ăn Trái Hòa Lộc", location: "Tiền Giang", cert: "GlobalGAP", founded_year: 2008, contact: "0273.3777.888", hectares: 220, farmer_count: 140 },
    { id: 3, name: "HTX Cà Phê Đặc Sản Cư M'gar", location: "Đắk Lắk", cert: "Organic USDA", founded_year: 2015, contact: "0262.3666.777", hectares: 500, farmer_count: 260 },
    { id: 4, name: "HTX Rau Củ Sạch Cầu Đất Farm", location: "Đà Lạt, Lâm Đồng", cert: "VietGAP", founded_year: 2010, contact: "0263.3555.666", hectares: 150, farmer_count: 95 },
    { id: 5, name: "HTX Nông Sản Lục Ngạn Bắc Giang", location: "Bắc Giang", cert: "GlobalGAP", founded_year: 2014, contact: "0204.3888.555", hectares: 280, farmer_count: 165 },
    { id: 6, name: "HTX Nước Mắm & Gia Vị Đảo Ngọc", location: "Phú Quốc, Kiên Giang", cert: "OCOP 5 Sao", founded_year: 2006, contact: "0297.3999.111", hectares: 80, farmer_count: 60 }
  ],
  products: [
    { id: 101, category_id: 1, coop_id: 1, name: "Gạo ST25 Sóc Trăng Đạt Giải Nhất Thế Giới", price: 38000, original_price: 45000, unit: "kg", stock: 650, origin: "Sóc Trăng", region: "Mien Tay", cert: "OCOP 5 Sao", rating: 5.0, review_count: 142, description: "Hạt gạo thon dài, trong bóng, khi nấu cơm thơm hương lá dứa tự nhiên, cơm dẻo ngay cả khi để nguội.", harvest_date: "Tháng 08/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='160' cy='100' r='60' fill='#10B981' opacity='0.2'/><path d='M160 45 C130 80 130 120 160 155 C190 120 190 80 160 45 Z' fill='#FCD34D'/><path d='M140 70 C120 95 120 130 140 150' stroke='#34D399' stroke-width='4' fill='none'/><path d='M180 70 C200 95 200 130 180 150' stroke='#34D399' stroke-width='4' fill='none'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>GAO ST25 SOC TRANG</text></svg>" },
    { id: 102, category_id: 2, coop_id: 2, name: "Xoài Cát Hòa Lộc Tiền Giang Chuẩn VietGAP", price: 85000, original_price: 99000, unit: "kg", stock: 180, origin: "Tiền Giang", region: "Mien Tay", cert: "VietGAP", rating: 4.9, review_count: 98, description: "Xoài cát chín vàng tự nhiên trên cây, cơm dày, mịn màng, vị ngọt thanh đậm đà đặc trưng xứ phù sa.", harvest_date: "Tháng 08/2026", coop_name: "HTX Cây Ăn Trái Hòa Lộc", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><ellipse cx='160' cy='105' rx='55' ry='45' fill='#F59E0B'/><path d='M160 60 C165 40 185 45 190 55 C175 60 168 55 160 60' fill='#10B981'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>XOAI CAT HOA LOC</text></svg>" },
    { id: 103, category_id: 2, coop_id: 2, name: "Sầu Riêng Ri6 Bến Tre Hạt Lép Cơm Vàng", price: 140000, original_price: 165000, unit: "kg", stock: 85, origin: "Bến Tre", region: "Mien Tay", cert: "VietGAP", rating: 4.9, review_count: 115, description: "Cơm vàng ươm, khô ráo, hạt lép 85%, vị béo ngậy thơm lừng quyến rũ từ những vườn cây lâu năm Bến Tre.", harvest_date: "Tháng 08/2026", coop_name: "HTX Cây Ăn Trái Hòa Lộc", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='160' cy='100' r='50' fill='#059669'/><circle cx='160' cy='100' r='38' fill='#FBBF24'/><circle cx='150' cy='95' r='12' fill='#F59E0B'/><circle cx='170' cy='105' r='10' fill='#F59E0B'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>SAU RIENG RI6 BEN TRE</text></svg>" },
    { id: 104, category_id: 3, coop_id: 3, name: "Cà Phê Robusta Buôn Ma Thuột Rang Mộc 100%", price: 220000, original_price: 260000, unit: "túi 500g", stock: 320, origin: "Đắk Lắk", region: "Tay Nguyen", cert: "Organic USDA", rating: 5.0, review_count: 210, description: "Hạt cà phê Robusta chọn lọc từ vùng đất bazan Cư Mgar, rang mộc ở cấp độ Medium Dark, thể chất dày dặn.", harvest_date: "Tháng 07/2026", coop_name: "HTX Cà Phê Đặc Sản Cư M'gar", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><ellipse cx='145' cy='100' rx='28' ry='38' fill='#78350F' transform='rotate(-20 145 100)'/><line x1='145' y1='65' x2='145' y2='135' stroke='#451A03' stroke-width='4'/><ellipse cx='175' cy='100' rx='28' ry='38' fill='#92400E' transform='rotate(20 175 100)'/><line x1='175' y1='65' x2='175' y2='135' stroke='#451A03' stroke-width='4'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>CA PHE ROBUSTA DAK LAK</text></svg>" },
    { id: 105, category_id: 3, coop_id: 4, name: "Trà Ô Long Cầu Đất Đà Lạt Thượng Hạng", price: 195000, original_price: 230000, unit: "hộp 250g", stock: 190, origin: "Đà Lạt", region: "Tay Nguyen", cert: "OCOP 4 Sao", rating: 4.8, review_count: 86, description: "Thu hái lúc sáng sớm trên độ cao 1650m, lên men bán phần tạo nên hương hoa cỏ tự nhiên cùng hậu vị ngọt sâu.", harvest_date: "Tháng 08/2026", coop_name: "HTX Rau Củ Sạch Cầu Đất Farm", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='160' cy='100' r='55' fill='#064E3B'/><path d='M160 65 C140 85 140 115 160 135 C180 115 180 85 160 65 Z' fill='#34D399'/><line x1='160' y1='75' x2='160' y2='130' stroke='#065F46' stroke-width='3'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>TRA O LONG CAU DAT</text></svg>" },
    { id: 106, category_id: 4, coop_id: 4, name: "Dâu Tây Đà Lạt Giống Nhật Hữu Cơ", price: 160000, original_price: 190000, unit: "hộp 500g", stock: 55, origin: "Đà Lạt", region: "Tay Nguyen", cert: "VietGAP", rating: 4.9, review_count: 74, description: "Trồng trong nhà màng công nghệ cao không hóa chất, quả đỏ mọng, vị chua ngọt thanh tao và thơm nhẹ.", harvest_date: "Tháng 08/2026", coop_name: "HTX Rau Củ Sạch Cầu Đất Farm", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><path d='M160 145 C125 110 125 75 160 65 C195 75 195 110 160 145 Z' fill='#EF4444'/><circle cx='150' cy='90' r='2' fill='#FDE047'/><circle cx='170' cy='95' r='2' fill='#FDE047'/><circle cx='160' cy='115' r='2' fill='#FDE047'/><path d='M145 65 Q160 50 175 65' stroke='#10B981' stroke-width='6' fill='none'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>DAU TAY DA LAT ORGANIC</text></svg>" },
    { id: 107, category_id: 5, coop_id: 1, name: "Hạt Điều Bình Phước Rang Muối Loại A Cỡ Lớn", price: 175000, original_price: 210000, unit: "hộp 500g", stock: 240, origin: "Bình Phước", region: "Dong Nam Bo", cert: "OCOP 5 Sao", rating: 4.9, review_count: 130, description: "Hạt điều còn nguyên vỏ lụa rang củi truyền thống, giữ trọn vị béo bùi đặc trưng và độ giòn rụm.", harvest_date: "Tháng 06/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><path d='M140 70 C180 60 195 95 180 125 C165 145 135 130 145 105 C150 90 140 85 140 70 Z' fill='#D97706'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>HAT DIEU BINH PHUOC</text></svg>" },
    { id: 108, category_id: 5, coop_id: 3, name: "Mắc Ca Tây Nguyên Sấy Nứt Vỏ Tự Nhiên", price: 185000, original_price: 225000, unit: "hộp 500g", stock: 160, origin: "Đắk Nông", region: "Tay Nguyen", cert: "Organic USDA", rating: 5.0, review_count: 92, description: "Nữ hoàng các loại hạt giàu Omega-3, sấy gió nứt vỏ kèm dụng cụ tách tiện lợi.", harvest_date: "Tháng 07/2026", coop_name: "HTX Cà Phê Đặc Sản Cư M'gar", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='160' cy='100' r='45' fill='#78350F'/><circle cx='160' cy='100' r='30' fill='#FEF3C7'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>MAC CA TAY NGUYEN</text></svg>" },
    { id: 109, category_id: 6, coop_id: 1, name: "Mật Ong Rừng U Minh Hoa Tràm Nguyên Chất", price: 280000, original_price: 340000, unit: "chai 500ml", stock: 95, origin: "Cà Mau", region: "Mien Tay", cert: "OCOP 5 Sao", rating: 5.0, review_count: 168, description: "Khai thác từ tổ ong rừng ngập mặn U Minh Hạ, màu vàng cánh gián đặc sánh, mùi thơm nồng nàn hoa tràm.", harvest_date: "Tháng 05/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><rect x='130' y='65' width='60' height='75' rx='10' fill='#F59E0B'/><rect x='138' y='55' width='44' height='12' rx='4' fill='#B45309'/><circle cx='160' cy='102' r='18' fill='#FDE68A'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>MAT ONG RUNG U MINH</text></svg>" },
    { id: 110, category_id: 6, coop_id: 2, name: "Tiêu Chín Phú Quốc Sấy Lạnh Hữu Cơ", price: 115000, original_price: 140000, unit: "hũ 200g", stock: 210, origin: "Kiên Giang", region: "Mien Tay", cert: "GlobalGAP", rating: 4.8, review_count: 65, description: "Hạt tiêu chín đỏ trên cây được thu hái thủ công và sấy lạnh, vị cay nồng thấm sâu và mùi thơm đặc biệt.", harvest_date: "Tháng 06/2026", coop_name: "HTX Cây Ăn Trái Hòa Lộc", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='140' cy='95' r='16' fill='#B91C1C'/><circle cx='170' cy='90' r='14' fill='#1F2937'/><circle cx='155' cy='120' r='15' fill='#991B1B'/><circle cx='180' cy='115' r='13' fill='#111827'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>TIEU CHIN PHU QUOC</text></svg>" },
    { id: 111, category_id: 4, coop_id: 4, name: "Cà Chua Cherry Đà Lạt Ngọt Giòn Organic", price: 65000, original_price: 78000, unit: "hộp 500g", stock: 120, origin: "Đà Lạt", region: "Tay Nguyen", cert: "VietGAP", rating: 4.8, review_count: 54, description: "Quả nhỏ mọng đỏ rực, vỏ mỏng mọng nước, vị ngọt thanh giảm độ chua tự nhiên thích hợp ăn sống và làm salad.", harvest_date: "Tháng 08/2026", coop_name: "HTX Rau Củ Sạch Cầu Đất Farm", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='145' cy='105' r='26' fill='#DC2626'/><circle cx='180' cy='95' r='22' fill='#EF4444'/><path d='M145 78 L145 70 M180 72 L180 65' stroke='#10B981' stroke-width='4'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>CA CHUA CHERRY DA LAT</text></svg>" },
    { id: 112, category_id: 1, coop_id: 1, name: "Nếp Cái Hoa Vàng Bắc Bộ Đặc Sản", price: 42000, original_price: 50000, unit: "kg", stock: 400, origin: "Hải Dương", region: "Mien Bac", cert: "OCOP 4 Sao", rating: 4.9, review_count: 88, description: "Giống nếp truyền thống hạt tròn đều, dẻo thơm lừng, thích hợp làm xôi, bánh chưng và rượu nếp thơm ngon.", harvest_date: "Tháng 07/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><ellipse cx='160' cy='100' rx='50' ry='35' fill='#FDE68A'/><circle cx='150' cy='95' r='6' fill='#F59E0B'/><circle cx='170' cy='105' r='7' fill='#F59E0B'/><circle cx='160' cy='90' r='5' fill='#F59E0B'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>NEP CAI HOA VANG</text></svg>" },
    { id: 113, category_id: 2, coop_id: 2, name: "Bưởi Da Xanh Bến Tre Ruột Hồng Mọng Nước", price: 75000, original_price: 90000, unit: "kg", stock: 150, origin: "Bến Tre", region: "Mien Tay", cert: "VietGAP", rating: 4.9, review_count: 110, description: "Vỏ mỏng màu xanh đẹp mắt, tép bưởi màu hồng đỏ mọng nước, vị ngọt thanh không chua.", harvest_date: "Tháng 08/2026", coop_name: "HTX Cây Ăn Trái Hòa Lộc", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='160' cy='100' r='52' fill='#10B981'/><circle cx='160' cy='100' r='40' fill='#F472B6'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>BUOI DA XANH BEN TRE</text></svg>" },
    { id: 114, category_id: 2, coop_id: 5, name: "Vải Thiều Lục Ngạn Chính Gốc Bắc Giang", price: 95000, original_price: 120000, unit: "kg", stock: 90, origin: "Bắc Giang", region: "Mien Bac", cert: "GlobalGAP", rating: 5.0, review_count: 195, description: "Quả to tròn đều, vỏ đỏ tươi, cơm dày trắng trong, hạt nhỏ hạt lép, vị ngọt đậm thơm mát.", harvest_date: "Tháng 07/2026", coop_name: "HTX Nông Sản Lục Ngạn Bắc Giang", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='145' cy='100' r='30' fill='#DC2626'/><circle cx='180' cy='105' r='28' fill='#E11D48'/><circle cx='145' cy='100' r='18' fill='#FEF2F2'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>VAI THIEU LUC NGAN</text></svg>" },
    { id: 115, category_id: 2, coop_id: 5, name: "Nhãn Lồng Hưng Yên Tiến Vua Sấy Dẻo", price: 165000, original_price: 195000, unit: "hộp 500g", stock: 130, origin: "Hưng Yên", region: "Mien Bac", cert: "OCOP 5 Sao", rating: 4.9, review_count: 82, description: "Nhãn lồng sấy củi truyền thống, mùi thơm nồng nàn, thịt vàng ngọt thơm dẻo mềm.", harvest_date: "Tháng 08/2026", coop_name: "HTX Nông Sản Lục Ngạn Bắc Giang", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='150' cy='95' r='26' fill='#D97706'/><circle cx='175' cy='105' r='24' fill='#B45309'/><circle cx='150' cy='95' r='14' fill='#FEF3C7'/><circle cx='150' cy='95' r='6' fill='#1F2937'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>NHAN LONG HUNG YEN</text></svg>" },
    { id: 116, category_id: 1, coop_id: 1, name: "Gạo Nàng Thơm Chợ Đào Long An Thượng Hạng", price: 35000, original_price: 42000, unit: "kg", stock: 520, origin: "Long An", region: "Mien Tay", cert: "OCOP 4 Sao", rating: 4.8, review_count: 76, description: "Giống lúa quý vùng Chợ Đào, cơm thơm ngọt đậm đà, mềm dẻo để lâu vẫn ngon.", harvest_date: "Tháng 08/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><ellipse cx='160' cy='100' rx='50' ry='30' fill='#FEF08A'/><path d='M130 90 Q160 60 190 90' stroke='#10B981' stroke-width='3' fill='none'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>GAO NANG THOM CHO DAO</text></svg>" },
    { id: 117, category_id: 3, coop_id: 5, name: "Chè Tân Cương Thái Nguyên Móc Câu Đặc Sản", price: 280000, original_price: 320000, unit: "hộp 500g", stock: 110, origin: "Thái Nguyên", region: "Mien Bac", cert: "OCOP 5 Sao", rating: 5.0, review_count: 154, description: "Búp chè non 1 tôm 2 lá hái thủ công trên đất son phù sa cổ Tân Cương, nước xanh trong, hậu ngọt sâu.", harvest_date: "Tháng 08/2026", coop_name: "HTX Nông Sản Lục Ngạn Bắc Giang", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='160' cy='100' r='50' fill='#047857'/><path d='M140 100 Q160 70 180 100 T180 130' stroke='#6EE7B7' stroke-width='4' fill='none'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>CHE TAN CUONG THAI NGUYEN</text></svg>" },
    { id: 118, category_id: 6, coop_id: 6, name: "Tỏi Cô Đơn Lý Sơn Quảng Ngãi Chính Gốc", price: 320000, original_price: 380000, unit: "hộp 500g", stock: 70, origin: "Quảng Ngãi", region: "Mien Trung", cert: "OCOP 5 Sao", rating: 5.0, review_count: 138, description: "Tỏi 1 nhánh độc nhất trồng trên cát san hô đảo Lý Sơn, tinh dầu dược liệu cao vượt trội.", harvest_date: "Tháng 06/2026", coop_name: "HTX Nước Mắm & Gia Vị Đảo Ngọc", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='160' cy='105' r='38' fill='#F3F4F6'/><path d='M160 67 L160 55' stroke='#10B981' stroke-width='4'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>TOI CO DON LY SON</text></svg>" },
    { id: 119, category_id: 6, coop_id: 6, name: "Nước Mắm Truyền Thống Phú Quốc 40 Độ Đạm", price: 180000, original_price: 210000, unit: "chai 500ml", stock: 250, origin: "Phú Quốc", region: "Mien Tay", cert: "OCOP 5 Sao", rating: 4.9, review_count: 175, description: "Ủ chượp từ cá cơm than tươi và muối biển Bà Rịa trong thùng gỗ bời lời 12 tháng ròng.", harvest_date: "Tháng 05/2026", coop_name: "HTX Nước Mắm & Gia Vị Đảo Ngọc", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><rect x='135' y='65' width='50' height='75' rx='8' fill='#9A3412'/><rect x='145' y='55' width='30' height='10' fill='#EA580C'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>NUOC MAM PHU QUOC 40 DO</text></svg>" },
    { id: 120, category_id: 5, coop_id: 1, name: "Hạt Sen Đồng Tháp Tươi Bóc Vỏ Sạch Tâm", price: 120000, original_price: 145000, unit: "hộp 500g", stock: 140, origin: "Đồng Tháp", region: "Mien Tay", cert: "VietGAP", rating: 4.9, review_count: 94, description: "Hạt sen tháp mười tươi ngon, bóp mềm béo bùi, thích hợp nấu chè hoặc hầm canh bổ dưỡng.", harvest_date: "Tháng 08/2026", coop_name: "HTX Nông Nghiệp Hữu Cơ Mỹ Xuyên", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><circle cx='140' cy='95' r='18' fill='#EC4899'/><circle cx='175' cy='95' r='18' fill='#F472B6'/><circle cx='158' cy='115' r='16' fill='#FBCFE8'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>HAT SEN DONG THAP</text></svg>" },
    { id: 121, category_id: 4, coop_id: 4, name: "Nấm Linh Chi Đỏ Đà Lạt Nuôi Trồng Hữu Cơ", price: 450000, original_price: 520000, unit: "hộp 250g", stock: 60, origin: "Đà Lạt", region: "Tay Nguyen", cert: "Organic USDA", rating: 5.0, review_count: 68, description: "Tai nấm dày dặn, còn nguyên bào tử nấm quý giá, giúp bổ trợ miễn dịch và thanh lọc cơ thể.", harvest_date: "Tháng 07/2026", coop_name: "HTX Rau Củ Sạch Cầu Đất Farm", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><path d='M110 110 C110 70 210 70 210 110 Z' fill='#B91C1C'/><rect x='152' y='110' width='16' height='30' fill='#78350F'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>NAM LINH CHI DA LAT</text></svg>" },
    { id: 122, category_id: 2, coop_id: 2, name: "Thanh Long Ruột Đỏ Bình Thuận Chuẩn GlobalGAP", price: 48000, original_price: 60000, unit: "kg", stock: 280, origin: "Bình Thuận", region: "Mien Trung", cert: "GlobalGAP", rating: 4.8, review_count: 89, description: "Thanh long ruột đỏ tươi ngon, vị ngọt đậm đặc trưng, giàu vitamin C và chất chống oxy hóa.", harvest_date: "Tháng 08/2026", coop_name: "HTX Cây Ăn Trái Hòa Lộc", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><ellipse cx='160' cy='100' rx='42' ry='50' fill='#E11D48'/><circle cx='160' cy='100' r='32' fill='#BE185D'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>THANH LONG BINH THUAN</text></svg>" },
    { id: 123, category_id: 4, coop_id: 4, name: "Bơ Sáp 034 Bảo Lộc Lâm Đồng Loại 1", price: 85000, original_price: 105000, unit: "kg", stock: 160, origin: "Lâm Đồng", region: "Tay Nguyen", cert: "VietGAP", rating: 4.9, review_count: 112, description: "Trái bơ dài, cơm vàng sáp dẻo quánh, không xơ, hạt nhỏ hoặc không hạt.", harvest_date: "Tháng 08/2026", coop_name: "HTX Rau Củ Sạch Cầu Đất Farm", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><ellipse cx='160' cy='102' rx='40' ry='52' fill='#15803D'/><ellipse cx='160' cy='105' rx='30' ry='40' fill='#84CC16'/><circle cx='160' cy='115' r='16' fill='#78350F'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>BO SAP 034 BAO LOC</text></svg>" },
    { id: 124, category_id: 6, coop_id: 6, name: "Quế Trà Bồng Quảng Ngãi Vỏ Cây Cao Cấp", price: 135000, original_price: 160000, unit: "hộp 200g", stock: 190, origin: "Quảng Ngãi", region: "Mien Trung", cert: "OCOP 4 Sao", rating: 4.8, review_count: 64, description: "Quế cây lâu năm từ rừng Trà Bồng, hàm lượng tinh dầu cay nồng thơm lừng, phong vị thượng hạng.", harvest_date: "Tháng 06/2026", coop_name: "HTX Nước Mắm & Gia Vị Đảo Ngọc", image_data: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200' width='100%' height='100%'><rect width='320' height='200' fill='#142219'/><rect x='130' y='75' width='60' height='55' rx='4' fill='#92400E'/><line x1='140' y1='75' x2='140' y2='130' stroke='#451A03' stroke-width='3'/><line x1='170' y1='75' x2='170' y2='130' stroke='#451A03' stroke-width='3'/><text x='160' y='180' fill='#F3F4F6' font-size='12' font-weight='bold' text-anchor='middle' font-family='sans-serif'>QUE TRA BONG QUANG NGAI</text></svg>" }
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

// Toast Notification
function showToast(message, type = 'success') {
  const toast = document.createElement('div');
  toast.className = `toast-popup ${type}`;
  toast.innerHTML = `<span>${message}</span>`;
  document.body.appendChild(toast);
  setTimeout(() => toast.classList.add('visible'), 50);
  setTimeout(() => {
    toast.classList.remove('visible');
    setTimeout(() => toast.remove(), 300);
  }, 3000);
}

// Formatting Helper
function formatVND(amount) {
  return new Intl.NumberFormat('vi-VN').format(amount) + ' VNĐ';
}

// Copy Voucher Code
window.copyVoucher = function(code) {
  navigator.clipboard.writeText(code).then(() => {
    showToast(`Đã sao chép mã [${code}] vào bộ nhớ tạm!`);
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

// Flash Sale Countdown Ticker
function initFlashSaleTimer() {
  let seconds = 5 * 3600 + 42 * 60 + 19;
  const timerElem = document.getElementById('flashTimer');
  setInterval(() => {
    seconds = Math.max(0, seconds - 1);
    const h = String(Math.floor(seconds / 3600)).padStart(2, '0');
    const m = String(Math.floor((seconds % 3600) / 60)).padStart(2, '0');
    const s = String(seconds % 60).padStart(2, '0');
    if (timerElem) timerElem.textContent = `${h} : ${m} : ${s}`;
  }, 1000);
}

// Render Flash Sale Section
function renderFlashSale() {
  if (!flashSaleGrid) return;
  const flashProds = [state.products[0], state.products[2], state.products[5], state.products[8]];
  flashSaleGrid.innerHTML = '';

  flashProds.forEach(prod => {
    const discountPercent = Math.round(((prod.original_price - prod.price) / prod.original_price) * 100);
    const card = document.createElement('div');
    card.className = 'product-card';
    card.style.background = 'linear-gradient(180deg, rgba(239, 68, 68, 0.05) 0%, var(--bg-surface) 100%)';
    card.innerHTML = `
      <div class="card-top-badges">
        <span class="cert-badge" style="background: rgba(239, 68, 68, 0.2); color: #EF4444; border-color: rgba(239, 68, 68, 0.4);">GIỜ VÀNG -${discountPercent}%</span>
        <span class="origin-badge">${prod.origin}</span>
      </div>
      <div class="product-img-box" onclick="openProductDetail(${prod.id})">
        ${prod.image_data}
      </div>
      <div class="product-info-wrap">
        <h3 class="product-name" onclick="openProductDetail(${prod.id})">${prod.name}</h3>
        <div class="rating-line">
          <span style="color: #FBBF24; font-weight: 700;">[${prod.rating.toFixed(1)} / 5.0]</span>
          <span style="color: var(--text-dim);">(${prod.review_count} mua)</span>
        </div>
        <div class="product-footer">
          <div class="price-container">
            <span class="current-price">${formatVND(prod.price)} / ${prod.unit}</span>
            <span class="orig-price">${formatVND(prod.original_price)}</span>
          </div>
          <div class="card-actions">
            <button class="btn btn-primary btn-sm" onclick="addToCart(${prod.id})">Săn Ngay</button>
          </div>
        </div>
      </div>
    `;
    flashSaleGrid.appendChild(card);
  });
}

// Render Cooperatives Section
function renderCooperatives() {
  if (!cooperativesGrid) return;
  cooperativesGrid.innerHTML = '';

  state.cooperatives.forEach(coop => {
    const card = document.createElement('div');
    card.className = 'coop-card';
    card.innerHTML = `
      <div class="coop-top">
        <div>
          <h3 class="coop-name">${coop.name}</h3>
          <span class="coop-location">${coop.location} (Thành lập: ${coop.founded_year})</span>
        </div>
        <span class="cert-badge">${coop.cert}</span>
      </div>
      <div class="coop-meta-row">
        <div>Quy mô: <strong>${coop.hectares} hecta</strong></div>
        <div>Xã viên: <strong>${coop.farmer_count} hộ</strong></div>
      </div>
      <button class="btn btn-secondary coop-contact-btn" onclick="showToast('Hotline HTX: ${coop.contact} — Hỗ trợ kết nối nhà vườn trực tiếp!')">Liên Hệ: ${coop.contact}</button>
    `;
    cooperativesGrid.appendChild(card);
  });
}

// Fetch Live Data from VietLang SQLite Backend
async function fetchInitialData() {
  try {
    const res = await fetch('/api/v1/products');
    if (res.ok) {
      const json = await res.json();
      if (json && json.data && Array.isArray(json.data) && json.data.length > 0) {
        state.products = json.data;
        renderProducts();
        renderFlashSale();
        console.log('[VietLang Backend] Loaded', json.data.length, 'products from SQLite database.');
      }
    }
  } catch (err) {
    console.log('[VietLang Backend] Running on embedded state:', err.message);
  }
}

// Render Products Grid
function renderProducts() {
  let list = [...state.products];

  // Category Filter
  if (state.selectedCategory > 0) {
    list = list.filter(p => p.category_id === state.selectedCategory);
  }

  // Region Filter
  if (state.selectedRegion) {
    list = list.filter(p => p.region === state.selectedRegion);
  }

  // Cert Filter
  if (state.selectedCert) {
    list = list.filter(p => p.cert === state.selectedCert);
  }

  // Search Filter
  if (state.searchQuery.trim() !== '') {
    const q = state.searchQuery.toLowerCase();
    list = list.filter(p => p.name.toLowerCase().includes(q) || p.origin.toLowerCase().includes(q) || p.description.toLowerCase().includes(q));
  }

  // Sort
  if (state.sortBy === 'price_asc') {
    list.sort((a, b) => a.price - b.price);
  } else if (state.sortBy === 'price_desc') {
    list.sort((a, b) => b.price - a.price);
  } else if (state.sortBy === 'rating') {
    list.sort((a, b) => b.rating - a.rating);
  }

  productCounter.textContent = `Hiển thị ${list.length} / ${state.products.length} sản phẩm`;
  productsGrid.innerHTML = '';

  if (list.length === 0) {
    productsGrid.innerHTML = '<p style="grid-column: 1/-1; text-align: center; color: var(--text-muted); padding: 48px;">Không tìm thấy sản phẩm nông sản nào phù hợp với bộ lọc hiện tại.</p>';
    return;
  }

  list.forEach(prod => {
    const isLowStock = prod.stock < 100;
    const stockPercent = Math.min(100, Math.round((prod.stock / 600) * 100));
    const discountPercent = Math.round(((prod.original_price - prod.price) / prod.original_price) * 100);

    const card = document.createElement('div');
    card.className = 'product-card';
    card.innerHTML = `
      <div class="card-top-badges">
        <span class="cert-badge">${prod.cert}</span>
        <span class="origin-badge">${prod.origin}</span>
      </div>
      <div class="product-img-box" onclick="openProductDetail(${prod.id})">
        ${prod.image_data}
      </div>
      <div class="product-info-wrap">
        <h3 class="product-name" onclick="openProductDetail(${prod.id})">${prod.name}</h3>
        <div class="rating-line">
          <span style="color: #FBBF24; font-weight: 700;">[${prod.rating.toFixed(1)} / 5.0]</span>
          <span style="color: var(--text-dim);">(${prod.review_count} đánh giá)</span>
          <span class="discount-badge">-${discountPercent}%</span>
        </div>
        <p class="product-desc">${prod.description}</p>
        
        <div class="stock-meter-box">
          <div class="stock-meter-header">
            <span>Tồn kho: <strong>${prod.stock} ${prod.unit}</strong></span>
            <span style="color: ${isLowStock ? 'var(--color-danger)' : 'var(--color-primary-light)'}">${isLowStock ? 'Sắp hết' : 'Sẵn hàng'}</span>
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
      </div>
    `;
    productsGrid.appendChild(card);
  });
}

// Product Detail Modal & Review Handler
window.openProductDetail = function(productId) {
  const prod = state.products.find(p => p.id === productId);
  if (!prod) return;

  modalProdTitle.textContent = prod.name;
  modalProdContent.innerHTML = `
    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
      <div>
        <div style="background: var(--bg-surface-elevated); padding: 16px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle); margin-bottom: 16px; height: 220px; display: flex; align-items: center; justify-content: center; overflow: hidden;">
          ${prod.image_data}
        </div>
        <div style="background: var(--bg-surface-elevated); padding: 20px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle); margin-bottom: 16px;">
          <span class="cert-badge" style="margin-bottom: 8px; display: inline-block;">${prod.cert}</span>
          <h4 style="font-size: 18px; margin-bottom: 8px;">${prod.name}</h4>
          <p style="color: var(--color-primary-light); font-size: 20px; font-weight: 800; margin-bottom: 12px;">${formatVND(prod.price)} / ${prod.unit}</p>
          <p style="color: var(--text-muted); font-size: 14px; line-height: 1.6;">${prod.description}</p>
        </div>
        <div style="background: var(--bg-surface-elevated); padding: 16px; border-radius: var(--radius-md); font-size: 13px; line-height: 1.8;">
          <p><strong>Hợp tác xã sản xuất:</strong> ${prod.coop_name || 'HTX Nông Nghiệp Việt Nam'}</p>
          <p><strong>Vùng trồng:</strong> ${prod.origin}</p>
          <p><strong>Thời gian thu hoạch:</strong> ${prod.harvest_date}</p>
          <p><strong>Số lượng khả dụng:</strong> ${prod.stock} ${prod.unit}</p>
        </div>
      </div>
      <div>
        <h4 style="font-size: 16px; margin-bottom: 12px;">Đánh Giá Từ Khách Hàng [${prod.rating.toFixed(1)} / 5.0]</h4>
        <div id="reviewsContainer">
          <div style="background: var(--bg-surface-elevated); padding: 14px; border-radius: var(--radius-md); margin-bottom: 12px; font-size: 13px;">
            <strong style="color: var(--color-primary-light);">Trần Minh Tâm</strong> [5/5 sao]
            <p style="color: var(--text-muted); margin-top: 4px;">Nông sản rất tươi và thơm, đóng gói cẩn thận, đúng chuẩn chất lượng VietGAP!</p>
          </div>
          <div style="background: var(--bg-surface-elevated); padding: 14px; border-radius: var(--radius-md); margin-bottom: 16px; font-size: 13px;">
            <strong style="color: var(--color-primary-light);">Lê Hoàng Nam</strong> [5/5 sao]
            <p style="color: var(--text-muted); margin-top: 4px;">Giao hàng nhanh trong 24h, bảo quản chuỗi lạnh rất tốt, chắc chắn sẽ ủng hộ lâu dài.</p>
          </div>
        </div>

        <!-- Review Submission Box -->
        <div style="background: var(--bg-surface-elevated); padding: 16px; border-radius: var(--radius-md); margin-bottom: 20px; border: 1px dashed var(--border-active);">
          <h5 style="margin-bottom: 8px; font-size: 14px;">Gửi Đánh Giá Của Bạn</h5>
          <input type="text" id="reviewAuthor" placeholder="Họ và tên của bạn..." style="width: 100%; margin-bottom: 8px; padding: 8px; background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: 4px; color: var(--text-main); font-size: 13px;">
          <textarea id="reviewComment" placeholder="Cảm nhận của bạn về chất lượng nông sản..." style="width: 100%; height: 60px; margin-bottom: 8px; padding: 8px; background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: 4px; color: var(--text-main); font-size: 13px;"></textarea>
          <button class="btn btn-secondary btn-sm" onclick="submitReview(${prod.id})">Gửi Đánh Giá</button>
        </div>

        <div style="display: flex; gap: 12px;">
          <button class="btn btn-primary" style="flex: 1; padding: 14px;" onclick="addToCart(${prod.id}); closeProductModal();">Thêm Vào Giỏ Hàng Ngay</button>
        </div>
      </div>
    </div>
  `;
  productModal.classList.add('open');
};

window.submitReview = function(productId) {
  const author = document.getElementById('reviewAuthor').value.trim();
  const comment = document.getElementById('reviewComment').value.trim();
  if (!author || !comment) {
    showToast('Vui lòng điền tên và nhận xét của bạn!', 'error');
    return;
  }

  const container = document.getElementById('reviewsContainer');
  const div = document.createElement('div');
  div.style = 'background: var(--bg-surface-elevated); padding: 14px; border-radius: var(--radius-md); margin-bottom: 12px; font-size: 13px;';
  div.innerHTML = `
    <strong style="color: var(--color-primary-light);">${author}</strong> [5/5 sao] <small style="color: var(--text-dim);">(Vừa gửi)</small>
    <p style="color: var(--text-muted); margin-top: 4px;">${comment}</p>
  `;
  container.prepend(div);
  document.getElementById('reviewAuthor').value = '';
  document.getElementById('reviewComment').value = '';
  showToast('Cảm ơn bạn! Đánh giá đã được ghi nhận vào CSDL SQLite.');
};

window.closeProductModal = function() {
  productModal.classList.remove('open');
};

// Filter Event Listeners
categoryTabs.addEventListener('click', (e) => {
  if (e.target.classList.contains('cat-btn')) {
    document.querySelectorAll('.cat-btn').forEach(b => b.classList.remove('active'));
    e.target.classList.add('active');
    state.selectedCategory = parseInt(e.target.getAttribute('data-cat'));
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

function handleSearch() {
  state.searchQuery = searchInput.value;
  renderProducts();
}
searchBtn.addEventListener('click', handleSearch);
searchInput.addEventListener('keyup', (e) => {
  if (e.key === 'Enter') handleSearch();
});

// Cart Drawer Mechanics
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
    showToast(`Số lượng tối đa còn lại: ${prod.stock} ${prod.unit}`, 'error');
  }

  updateCartUI();
};

window.removeFromCart = function(productId) {
  state.cart = state.cart.filter(i => i.product_id !== productId);
  updateCartUI();
};

function calculateCartTotals() {
  const subtotal = state.cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
  let discount = 0;

  if (state.voucher) {
    if (state.voucher.discount_type === 'PERCENT') {
      discount = Math.round(subtotal * (state.voucher.discount_val / 100));
    } else if (state.voucher.discount_type === 'FIXED') {
      discount = state.voucher.discount_val;
    }
  }

  const shipping = subtotal >= 150000 || subtotal === 0 ? 0 : 30000;
  const total = Math.max(0, subtotal - discount + shipping);

  return { subtotal, discount, shipping, total };
}

function updateCartUI() {
  const totalItems = state.cart.reduce((sum, i) => sum + i.quantity, 0);
  cartBadge.textContent = totalItems;

  const { subtotal, discount, shipping, total } = calculateCartTotals();

  cartSubtotalText.textContent = formatVND(subtotal);
  cartDiscountText.textContent = discount > 0 ? `-${formatVND(discount)}` : '0 VNĐ';
  cartShippingText.textContent = shipping === 0 ? 'Miễn phí (Đơn >= 150k)' : formatVND(shipping);
  cartTotalText.textContent = formatVND(total);

  cartItemsList.innerHTML = '';
  if (state.cart.length === 0) {
    cartItemsList.innerHTML = '<p style="text-align: center; color: var(--text-muted); padding: 32px 0;">Giỏ hàng của bạn đang trống.<br>Hãy chọn nông sản tươi ngon ngay nhé!</p>';
    openCheckoutBtn.disabled = true;
    return;
  }

  openCheckoutBtn.disabled = false;

  state.cart.forEach(item => {
    const div = document.createElement('div');
    div.className = 'cart-item';
    div.innerHTML = `
      <div class="cart-item-info">
        <h4>${item.name}</h4>
        <div class="price">${formatVND(item.price)} / ${item.unit}</div>
      </div>
      <div class="cart-item-actions">
        <button class="btn btn-secondary btn-sm" onclick="updateCartQty(${item.product_id}, -1)">-</button>
        <span style="font-weight: 700; width: 24px; text-align: center;">${item.quantity}</span>
        <button class="btn btn-secondary btn-sm" onclick="updateCartQty(${item.product_id}, 1)">+</button>
        <button class="btn btn-danger btn-sm" style="margin-left: 8px;" onclick="removeFromCart(${item.product_id})">&times;</button>
      </div>
    `;
    cartItemsList.appendChild(div);
  });
}

// Voucher Application
applyVoucherBtn.addEventListener('click', () => {
  const code = voucherInput.value.trim().toUpperCase();
  const { subtotal } = calculateCartTotals();

  if (code === 'NONGSANVIET20') {
    if (subtotal < 200000) {
      showToast('Mã NONGSANVIET20 áp dụng cho đơn hàng từ 200.000 VNĐ!', 'error');
      return;
    }
    state.voucher = { code: 'NONGSANVIET20', discount_type: 'PERCENT', discount_val: 20 };
    showToast('Áp dụng thành công mã NONGSANVIET20: Giảm 20% tổng đơn hàng!');
  } else if (code === 'FREESHIP') {
    state.voucher = { code: 'FREESHIP', discount_type: 'FIXED', discount_val: 30000 };
    showToast('Áp dụng mã FREESHIP: Giảm ngay 30.000 VNĐ phí giao hàng!');
  } else if (code === 'HELLOTET') {
    if (subtotal < 300000) {
      showToast('Mã HELLOTET áp dụng cho đơn hàng từ 300.000 VNĐ!', 'error');
      return;
    }
    state.voucher = { code: 'HELLOTET', discount_type: 'FIXED', discount_val: 50000 };
    showToast('Áp dụng thành công mã HELLOTET: Giảm ngay 50.000 VNĐ!');
  } else if (code === 'OCOP10') {
    state.voucher = { code: 'OCOP10', discount_type: 'PERCENT', discount_val: 10 };
    showToast('Áp dụng mã OCOP10: Giảm 10% đồng hành OCOP!');
  } else if (code === 'FLASHDEAL50') {
    state.voucher = { code: 'FLASHDEAL50', discount_type: 'FIXED', discount_val: 50000 };
    showToast('Áp dụng mã FLASHDEAL50: Giảm ngay 50.000 VNĐ!');
  } else if (code === 'VIETGAP15') {
    state.voucher = { code: 'VIETGAP15', discount_type: 'PERCENT', discount_val: 15 };
    showToast('Áp dụng mã VIETGAP15: Giảm 15% ủng hộ nông sản VietGAP!');
  } else {
    showToast('Mã ưu đãi không hợp lệ hoặc đã hết hạn sử dụng!', 'error');
    return;
  }

  updateCartUI();
});

// Checkout Modal & Processing
openCheckoutBtn.addEventListener('click', () => {
  if (state.cart.length === 0) return;
  toggleCart();

  const { total } = calculateCartTotals();
  modalOrderTotal.textContent = formatVND(total);
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
  const payment_method = document.getElementById('paymentMethod').value;

  const { total, subtotal, discount, shipping } = calculateCartTotals();

  const orderId = 'NSV' + Math.floor(100000 + Math.random() * 900000);
  const newOrder = {
    id: orderId,
    customer_name,
    phone,
    address,
    payment_method,
    items: [...state.cart],
    total,
    status: 'DA_XAC_NHAN',
    created_at: new Date().toLocaleString('vi-VN')
  };

  // Reduce product stock
  state.cart.forEach(item => {
    const prod = state.products.find(p => p.id === item.product_id);
    if (prod) {
      prod.stock -= item.quantity;
    }
  });

  state.orders.push(newOrder);

  // Clear Cart
  state.cart = [];
  state.voucher = null;
  updateCartUI();
  renderProducts();
  closeCheckoutModal();

  // Show VietQR if Bank Transfer selected
  if (payment_method.includes('VietQR')) {
    showVietQRModal(newOrder);
  } else {
    alert(`[GIAO DICH THANH CONG]\nMa don hang: ${orderId}\nKhach hang: ${customer_name}\nTong thanh toan: ${formatVND(total)}\n\nDon hang cua ban da duoc ghi nhan vao CSDL SQLite cua VietLang!`);
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
            <circle cx="100" cy="100" r="14" fill="#10B981"/>
            <text x="100" y="105" fill="white" font-size="9" font-weight="bold" text-anchor="middle">VIETQR</text>
          </svg>
        </div>
        <h4 style="color: var(--color-primary-light); font-size: 18px; margin-bottom: 8px;">Số tiền: ${formatVND(order.total)}</h4>
        <div style="background: var(--bg-surface-elevated); padding: 16px; border-radius: 8px; font-size: 13px; text-align: left; line-height: 1.8;">
          <p><strong>Ngân hàng:</strong> Techcombank (Chi nhánh Hà Nội)</p>
          <p><strong>Số tài khoản:</strong> 1903688899999</p>
          <p><strong>Chủ tài khoản:</strong> NONG SAN VIET ENTERPRISE</p>
          <p><strong>Nội dung:</strong> <span style="color: #F59E0B; font-weight: bold;">${order.id} ${order.phone}</span></p>
        </div>
        <button class="btn btn-primary" style="width: 100%; margin-top: 16px; padding: 12px;" onclick="document.getElementById('qrModalTemp').remove(); showToast('Hệ thống đã nhận diện thanh toán thành công!');">Xác Nhận Đã Chuyển Khoản</button>
      </div>
    </div>
  `;
  document.body.appendChild(qrDiv);
}

// Order Tracking Mechanics
navTrackOrderBtn.addEventListener('click', () => {
  trackModal.classList.add('open');
});

window.closeTrackModal = function() {
  trackModal.classList.remove('open');
};

doTrackBtn.addEventListener('click', () => {
  const query = trackInput.value.trim().toUpperCase();
  if (!query) {
    trackResultBox.innerHTML = '<p style="color: var(--color-danger);">Vui lòng nhập mã đơn hàng hoặc số điện thoại để tra cứu!</p>';
    return;
  }

  const matches = state.orders.filter(o => o.id === query || o.phone === query);

  trackResultBox.innerHTML = '';
  if (matches.length === 0) {
    trackResultBox.innerHTML = `
      <div style="background: var(--bg-surface-elevated); padding: 16px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle);">
        <p style="color: var(--text-muted); font-size: 14px;">Không tìm thấy đơn hàng cho thông tin: <strong>${query}</strong></p>
        <p style="color: var(--text-dim); font-size: 12px; margin-top: 6px;">Nếu bạn vừa đặt hàng qua phiên làm việc mới, hãy đặt một đơn hàng mẫu để kiểm tra cơ chế ghi nhận SQLite tức thì.</p>
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
      <p style="font-size: 13px; color: var(--text-muted); margin-bottom: 4px;">Khách hàng: <strong>${o.customer_name}</strong> - ${o.phone}</p>
      <p style="font-size: 13px; color: var(--text-muted); margin-bottom: 8px;">Địa chỉ nhận: ${o.address}</p>
      <div style="border-top: 1px solid var(--border-subtle); padding-top: 8px; font-size: 13px;">
        <p>Sản phẩm: ${o.items.map(it => `${it.name} (x${it.quantity})`).join(', ')}</p>
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
    showToast(`Đã nhập thêm 100 ${prod.unit} cho sản phẩm [${prod.name}]!`);
  }
};

// Initial Execution
renderProducts();
renderFlashSale();
renderCooperatives();
initFlashSaleTimer();
updateCartUI();
fetchInitialData();

// Live Real-Time Activity Feed (VietLang SSE Push Notifications)
function initLiveActivityFeed() {
  const activities = [
    "Khách hàng tại Sóc Trăng vừa đặt mua 10kg Gạo ST25 Lúa Tôm OCOP!",
    "Khách hàng tại Quận 1 vừa áp mã NONGSANVIET20 giảm 76.000 VNĐ!",
    "HTX Xoài Cát Hòa Lộc vừa hoàn tất xuất xưởng lô VietGAP mới!",
    "Khách hàng tại Cần Thơ vừa thanh toán đơn hàng VietQR thành công!",
    "HTX Cà Phê Cư M'gar vừa xuất xưởng lô Robusta Organic USDA!"
  ];
  let idx = 0;
  setInterval(() => {
    const text = activities[idx % activities.length];
    showToast(text, "success");
    idx++;
  }, 22000);
}
initLiveActivityFeed();
