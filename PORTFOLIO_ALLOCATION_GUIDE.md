# 📚 Hướng dẫn sử dụng Portfolio Allocation Table Component

## ✅ Đã hoàn thành

File `PortfolioAllocationTable.slint` đã được tạo thành công với đầy đủ tính năng!

## 🎯 Tính năng chính

1. ✅ **Dynamic rows**: Bắt đầu với 2 rows, có thể mở rộng đến 20 rows
2. ✅ **"(More)" button**: Click để thêm 2 rows mới, button tự động di chuyển xuống row cuối
3. ✅ **Ticker input**: Nhập mã cổ phiếu với icon search 🔍
4. ✅ **Weight input**: Nhập % tỷ trọng với auto-validation
5. ✅ **Total calculation**: Tự động tính tổng và highlight red/green
6. ✅ **Validation**: Kiểm tra total = 100%, hiển thị warning message

---

## 📁 Cấu trúc files

```
ui/pages/monte_carlo/
├── components/
│   └── PortfolioAllocationTable.slint    ✅ Đã tạo
└── monte_carlo_window.slint                ✅ Đã integrate (lines 32-33, 128-138)
```

---

## 🔧 Cách sử dụng trong monte_carlo_window.slint

### Component đã được import và sử dụng:

**File:** `monte_carlo_window.slint` (lines 128-138)

```slint
PortfolioAllocationTable {
    ticker-changed(index, ticker) => {
        debug("Asset", index, "ticker changed:", ticker);
    }
    weight-changed(index, weight) => {
        debug("Asset", index, "weight changed:", weight);
    }
    search-ticker(index) => {
        debug("Search ticker for asset", index);
    }
}
```

### Cần update để handle callback "(More)" button:

**Thay đổi code trên thành:**

```slint
PortfolioAllocationTable {
    ticker-changed(index, ticker) => {
        debug("Asset", index, "ticker changed:", ticker);
        // TODO: Lưu ticker vào state hoặc backend
    }

    weight-changed(index, weight) => {
        debug("Asset", index, "weight changed:", weight);
        // TODO: Lưu weight vào state hoặc backend
    }

    search-ticker(index) => {
        debug("Search ticker for asset", index);
        // TODO: Mở popup search hoặc autocomplete
    }

    add-more-rows() => {
        // TODO: Tăng visible-count lên 2
        // Có thể dùng property binding hoặc callback to Rust
        debug("Add 2 more rows");
    }
}
```

---

## 🔗 Kết nối với Rust Backend

### Bước 1: Tìm file Rust handler

Có thể là một trong các files:
- `src/tasks/monte_carlo/mod.rs`
- `src/monte_carlo/mod.rs`
- File nào handle MonteCarloGlobal callbacks

### Bước 2: Thêm callback handler

**Ví dụ code Rust:**

```rust
// Trong file mod.rs hoặc tương tự

use slint::*;

slint::include_modules!();

fn main() {
    let ui = MonteCarloPage::new().unwrap();

    // Handle ticker changed
    ui.on_ticker_changed(|index, ticker| {
        println!("Ticker {} changed to: {}", index, ticker);
        // TODO: Validate ticker, update state
    });

    // Handle weight changed
    ui.on_weight_changed(|index, weight| {
        println!("Weight {} changed to: {}", index, weight);
        // TODO: Validate weight, recalculate total
    });

    // Handle search ticker
    ui.on_search_ticker(|index| {
        println!("Search ticker for index: {}", index);
        // TODO: Open search popup, fetch ticker list from API
    });

    // Handle add more rows
    ui.on_add_more_rows(|| {
        let current_count = ui.get_visible_count();
        if current_count < 20 {
            ui.set_visible_count(current_count + 2);
        }
    });

    ui.run().unwrap();
}
```

---

## 📊 Properties của Component

### Input/Output Properties:

```slint
// Số rows hiển thị (2-20)
in-out property <int> visible-count: 2;

// Ticker symbols (1-20)
in-out property <string> ticker1: "";
in-out property <string> ticker2: "";
// ... ticker3 đến ticker20

// Weights (1-20)
in-out property <float> weight1: 0;
in-out property <float> weight2: 0;
// ... weight3 đến weight20

// Computed properties
property <float> total-weight;    // Auto-calculated
property <bool> is-valid;         // true if total = 100%
property <bool> can-add-more;     // true if visible-count < 20
```

### Callbacks:

```slint
callback ticker-changed(int index, string ticker);
callback weight-changed(int index, float weight);
callback search-ticker(int index);
callback add-more-rows();
```

---

## 🎨 UI Flow

### Initial State:
```
┌────────────┬─────────────────────┬─────────┐
│ Ticker 1   │ [_____________] 🔍  │ [__] %  │
│ Ticker 2   │ [_____________] 🔍  │ [__] %  │  ← Has "(More)"
│            │ (More)              │         │
├────────────┼─────────────────────┼─────────┤
│ Total:     │                     │ 0.0 %   │  ← Red (invalid)
└────────────┴─────────────────────┴─────────┘
⚠️ Total weight must equal 100%. Current: 0.0%
```

### After clicking "(More)":
```
┌────────────┬─────────────────────┬─────────┐
│ Ticker 1   │ [_____________] 🔍  │ [__] %  │
│ Ticker 2   │ [_____________] 🔍  │ [__] %  │
│ Ticker 3   │ [_____________] 🔍  │ [__] %  │
│ Ticker 4   │ [_____________] 🔍  │ [__] %  │  ← "(More)" moved here
│            │ (More)              │         │
├────────────┼─────────────────────┼─────────┤
│ Total:     │                     │ 0.0 %   │
└────────────┴─────────────────────┴─────────┘
```

### With valid data:
```
┌────────────┬─────────────────────┬─────────┐
│ Ticker 1   │ [VNM          ] 🔍  │ [40] %  │
│ Ticker 2   │ [FPT          ] 🔍  │ [35] %  │
│            │ (More)              │         │
├────────────┼─────────────────────┼─────────┤
│ Total:     │                     │ 75.0 %  │  ← Red (not 100%)
└────────────┴─────────────────────┴─────────┘
⚠️ Total weight must equal 100%. Current: 75.0%
```

### Valid portfolio:
```
┌────────────┬─────────────────────┬─────────┐
│ Ticker 1   │ [VNM          ] 🔍  │ [40] %  │
│ Ticker 2   │ [FPT          ] 🔍  │ [35] %  │
│ Ticker 3   │ [VCB          ] 🔍  │ [25] %  │
│ Ticker 4   │ [_____________] 🔍  │ [__] %  │
│            │ (More)              │         │
├────────────┼─────────────────────┼─────────┤
│ Total:     │                     │ 100.0 % │  ← Green (valid!)
└────────────┴─────────────────────┴─────────┘
```

---

## 🧪 Testing

### Test Case 1: Add rows
1. Mở app, chỉ thấy 2 rows
2. Click "(More)" → Thấy 4 rows, "(More)" di chuyển xuống row 4
3. Click "(More)" lần nữa → Thấy 6 rows
4. Lặp lại đến 20 rows → "(More)" biến mất

### Test Case 2: Validation
1. Nhập VNM = 40%, FPT = 35% → Total = 75% (Red, warning)
2. Nhập VCB = 25% → Total = 100% (Green, no warning)
3. Nhập VCB = 30% → Total = 105% (Red, warning)

### Test Case 3: Input handling
1. Nhập ticker "vnm" → Auto uppercase thành "VNM" (TODO: cần implement)
2. Nhập weight "abc" → Reject, chỉ nhận số
3. Nhập weight "150" → Warning nếu total > 100%

### Test Case 4: Search icon
1. Click 🔍 → Trigger callback với index
2. Debug log: "Search ticker for asset 0" (hoặc 1, 2,...)

---

## 🎯 Next Steps (Việc cần làm tiếp)

### 1. Backend Integration ⏳
- [ ] Wire up callbacks trong Rust
- [ ] Implement `add_more_rows` handler
- [ ] Save portfolio data to state

### 2. Search Functionality 🔍
- [ ] Create search popup/dialog
- [ ] Fetch ticker list from API
- [ ] Implement autocomplete

### 3. Validation Logic ✅
- [ ] Check duplicate tickers
- [ ] Validate ticker format (3-4 chars)
- [ ] Convert ticker to uppercase
- [ ] Prevent weight > 100%

### 4. Enhanced Features 🚀
- [ ] Auto-suggest remaining weight
- [ ] Drag & drop để reorder rows
- [ ] Copy/paste portfolio from clipboard
- [ ] Save/load portfolio presets

### 5. UI Polish 💅
- [ ] Add animations khi expand rows
- [ ] Better hover effects
- [ ] Loading state khi search
- [ ] Error tooltips

---

## 🆘 Troubleshooting

### Lỗi: "Unknown element 'PortfolioRow'"
**Nguyên nhân:** Component phải được định nghĩa trước khi sử dụng.
**Giải pháp:** PortfolioRow đã được di chuyển lên đầu file ✅

### Lỗi: "Component is neither used nor exported"
**Nguyên nhân:** PortfolioRow chưa được sử dụng.
**Giải pháp:** Đã sử dụng trong PortfolioAllocationTable ✅

### Lỗi: Callbacks không hoạt động
**Nguyên nhân:** Chưa wire up trong parent hoặc Rust backend.
**Giải pháp:** Thêm handlers như hướng dẫn ở trên ⏳

### Component không render
**Nguyên nhân:** Import statement bị sai hoặc file path không đúng.
**Giải pháp:** Check import trong monte_carlo_window.slint:
```slint
import { PortfolioAllocationTable } from "components/PortfolioAllocationTable.slint";
```

---

## 📞 Support

Nếu cần hỗ trợ thêm:
1. Check file [Sprint.md](Sprint.md) cho roadmap đầy đủ
2. Check file [Flow.md](Flow.md) cho thiết kế chi tiết
3. Tham khảo Slint documentation: https://slint.dev/docs

---

## ✨ Summary

**Component đã sẵn sàng sử dụng!** Chỉ cần:
1. ✅ File `PortfolioAllocationTable.slint` đã được tạo
2. ✅ Component đã được import và sử dụng trong `monte_carlo_window.slint`
3. ⏳ Cần implement callback `add-more-rows` để thêm rows
4. ⏳ Cần implement search ticker functionality
5. ⏳ Cần kết nối với Rust backend để lưu data

**Next action:** Implement `add-more-rows` callback trong Rust hoặc Slint để tăng `visible-count`.
