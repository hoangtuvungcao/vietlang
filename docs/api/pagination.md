# Module `std.pagination`

Module: std.pagination

## Quickstart

```vietlang
import std.pagination
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn pagination_new(page: Int = 1, page_size: Int = 10, total_items: Int = 0)` | Tao metadata phan trang cho REST API va Database @param page: Int - So thu tu trang hien tai (bat dau tu 1) @param page_size: Int - So luong phan tu tren moi trang (1-100) @param total_items: Int - Tong so ban ghi trong CSDL @return Map - Metadata bao gom offset, limit, total_pages, has_next, has_prev |
| `fn pagination_slice(items_array, page: Int = 1, page_size: Int = 10)` | Cat mang du lieu theo trang va kich thuoc trang @param items_array: Array - Danh sach phan tu can cat @param page: Int - So trang @param page_size: Int - So phan tu moi trang @return Map - Chua danh sach items da cat va metadata pagination |

---

### Function Details

#### `fn pagination_new(page: Int = 1, page_size: Int = 10, total_items: Int = 0)`

Tao metadata phan trang cho REST API va Database @param page: Int - So thu tu trang hien tai (bat dau tu 1) @param page_size: Int - So luong phan tu tren moi trang (1-100) @param total_items: Int - Tong so ban ghi trong CSDL @return Map - Metadata bao gom offset, limit, total_pages, has_next, has_prev

#### `fn pagination_slice(items_array, page: Int = 1, page_size: Int = 10)`

Cat mang du lieu theo trang va kich thuoc trang @param items_array: Array - Danh sach phan tu can cat @param page: Int - So trang @param page_size: Int - So phan tu moi trang @return Map - Chua danh sach items da cat va metadata pagination

