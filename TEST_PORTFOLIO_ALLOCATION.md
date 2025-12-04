# 🧪 Test Guide - Portfolio Allocation Table

## ✅ Implementation Status

**Callback `add-more-rows` đã được implement thành công!**

---

## 📦 Changes Made

### 1. **monte_carlo_window.slint** (Line 73)
Added property:
```slint
in-out property <int> portfolio-visible-count: 2;  // Start with 2 rows
```

### 2. **monte_carlo_window.slint** (Lines 131-166)
Updated PortfolioAllocationTable usage:
```slint
PortfolioAllocationTable {
    // Bind visible-count to parent property
    visible-count <=> root.portfolio-visible-count;

    // Handle callbacks...
    add-more-rows() => {
        // Add 2 more rows (maximum 20)
        if (root.portfolio-visible-count < 20) {
            root.portfolio-visible-count = Math.min(
                root.portfolio-visible-count + 2,
                20
            );
            debug("Added 2 rows. Total visible:", root.portfolio-visible-count);
        } else {
            debug("Maximum 20 tickers reached!");
        }
    }
}
```

---

## 🚀 How to Test

### Step 1: Run the app
```bash
cd c:\AIM-Trading\AIM-Trading_dev
cargo run
```

### Step 2: Navigate to Monte Carlo page
1. App sẽ mở và hiển thị Monte Carlo Simulation page
2. Đảm bảo đang ở mode "📊 Single Ticker" (button màu xanh)

### Step 3: Test "(More)" button

**Initial state:**
- Bạn sẽ thấy 2 rows:
  - `Ticker 1` với input fields
  - `Ticker 2(More)` với button "(More)" màu xanh

**Test sequence:**

1. **Click lần 1 vào "(More)"**
   - ✅ Expected: 2 rows mới xuất hiện (Ticker 3, Ticker 4)
   - ✅ Expected: "(More)" di chuyển xuống row 4
   - ✅ Expected: Console log: `"Added 2 rows. Total visible: 4"`

2. **Click lần 2 vào "(More)"**
   - ✅ Expected: 2 rows mới xuất hiện (Ticker 5, Ticker 6)
   - ✅ Expected: "(More)" di chuyển xuống row 6
   - ✅ Expected: Console log: `"Added 2 rows. Total visible: 6"`

3. **Continue clicking...**
   - ✅ Expected: Mỗi lần click thêm 2 rows
   - ✅ Expected: "(More)" luôn ở row cuối cùng

4. **Click đến row 20**
   - ✅ Expected: Khi đạt 20 rows, "(More)" biến mất
   - ✅ Expected: Row 20 không có "(More)"
   - ✅ Expected: Message: "ℹ️ Maximum 20 tickers reached"

### Step 4: Test input fields

**Test ticker input:**
1. Click vào ticker input field của row 1
2. Nhập "VNM"
3. ✅ Expected: Console log: `"Asset 0 ticker changed: VNM"`

**Test weight input:**
1. Click vào weight input field của row 1
2. Nhập "40"
3. ✅ Expected: Console log: `"Asset 0 weight changed: 40"`
4. ✅ Expected: Total row cập nhật thành "40.0%"

**Test search icon:**
1. Click vào icon 🔍 ở row 1
2. ✅ Expected: Console log: `"Search ticker for asset 0"`

### Step 5: Test validation

**Test total calculation:**
1. Nhập ticker1 = "VNM", weight1 = 40
2. Nhập ticker2 = "FPT", weight2 = 35
3. ✅ Expected: Total = 75.0% (màu đỏ)
4. ✅ Expected: Warning: "⚠️ Total weight must equal 100%. Current: 75.0%"

5. Nhập ticker3 = "VCB", weight3 = 25
6. ✅ Expected: Total = 100.0% (màu xanh)
7. ✅ Expected: No warning message

---

## 🐛 Debug Console Output Examples

### Successful "(More)" clicks:
```
Added 2 rows. Total visible: 4
Added 2 rows. Total visible: 6
Added 2 rows. Total visible: 8
...
Added 2 rows. Total visible: 20
Maximum 20 tickers reached!
```

### Input changes:
```
Asset 0 ticker changed: VNM
Asset 0 weight changed: 40
Asset 1 ticker changed: FPT
Asset 1 weight changed: 35
Asset 2 ticker changed: VCB
Asset 2 weight changed: 25
```

### Search icon clicks:
```
Search ticker for asset 0
Search ticker for asset 1
Search ticker for asset 2
```

---

## ✅ Expected UI States

### State 1: Initial (2 rows)
```
┌──────────────┬─────────────────────┬─────────┐
│ Ticker 1     │ [___________] 🔍    │ [__] %  │
│ Ticker 2     │ [___________] 🔍    │ [__] %  │
│     (More)   │                     │         │  ← Click here
├──────────────┼─────────────────────┼─────────┤
│ Total:       │                     │ 0.0 %   │  ← Red
└──────────────┴─────────────────────┴─────────┘
⚠️ Total weight must equal 100%. Current: 0.0%
```

### State 2: After 1st click (4 rows)
```
┌──────────────┬─────────────────────┬─────────┐
│ Ticker 1     │ [___________] 🔍    │ [__] %  │
│ Ticker 2     │ [___________] 🔍    │ [__] %  │
│ Ticker 3     │ [___________] 🔍    │ [__] %  │
│ Ticker 4     │ [___________] 🔍    │ [__] %  │
│     (More)   │                     │         │  ← Moved here
├──────────────┼─────────────────────┼─────────┤
│ Total:       │                     │ 0.0 %   │
└──────────────┴─────────────────────┴─────────┘
```

### State 3: With valid data
```
┌──────────────┬─────────────────────┬─────────┐
│ Ticker 1     │ [VNM        ] 🔍    │ [40] %  │
│ Ticker 2     │ [FPT        ] 🔍    │ [35] %  │
│ Ticker 3     │ [VCB        ] 🔍    │ [25] %  │
│ Ticker 4     │ [___________] 🔍    │ [__] %  │
│     (More)   │                     │         │
├──────────────┼─────────────────────┼─────────┤
│ Total:       │                     │ 100.0 % │  ← Green!
└──────────────┴─────────────────────┴─────────┘
✓ Valid portfolio!
```

### State 4: Maximum rows (20)
```
┌──────────────┬─────────────────────┬─────────┐
│ Ticker 1     │ [___________] 🔍    │ [__] %  │
│ Ticker 2     │ [___________] 🔍    │ [__] %  │
│ ...          │ ...                 │ ...     │
│ Ticker 19    │ [___________] 🔍    │ [__] %  │
│ Ticker 20    │ [___________] 🔍    │ [__] %  │  ← No "(More)"
├──────────────┼─────────────────────┼─────────┤
│ Total:       │                     │ 0.0 %   │
└──────────────┴─────────────────────┴─────────┘
ℹ️ Maximum 20 tickers reached
```

---

## 🎯 Test Checklist

### Basic Functionality
- [ ] App runs without errors
- [ ] PortfolioAllocationTable component renders
- [ ] Initial state shows 2 rows
- [ ] "(More)" button visible on row 2

### "(More)" Button
- [ ] Click "(More)" adds 2 new rows
- [ ] "(More)" button moves to last row
- [ ] Clicking multiple times works correctly
- [ ] Maximum 20 rows enforced
- [ ] "(More)" disappears at row 20
- [ ] Message appears when max reached

### Input Fields
- [ ] Ticker input accepts text
- [ ] Weight input accepts numbers only
- [ ] Decimal numbers work (e.g., 40.5)
- [ ] Console logs show correct values

### Search Icon
- [ ] 🔍 icon clickable
- [ ] Click triggers callback
- [ ] Console log shows correct index

### Validation
- [ ] Total calculates automatically
- [ ] Total = 100% shows green
- [ ] Total ≠ 100% shows red
- [ ] Warning message appears/disappears correctly

### Edge Cases
- [ ] Empty inputs handled correctly
- [ ] Zero weights displayed properly
- [ ] Very small weights (0.1%) work
- [ ] Large weights (99.9%) work

---

## 🆘 Troubleshooting

### Issue 1: "(More)" button không hoạt động
**Check:**
- Console có báo lỗi không?
- Property `portfolio-visible-count` có được bind không?
- Callback `add-more-rows()` có được define không?

**Fix:**
Xem lại lines 133 và 154-165 trong monte_carlo_window.slint

### Issue 2: Total không tự động tính
**Reason:**
Total được tính trong PortfolioAllocationTable component (auto)

**Check:**
Có thể do weight input không parse đúng. Check console log.

### Issue 3: Rows không xuất hiện sau khi click
**Check:**
1. Debug console có log "Added 2 rows..." không?
2. Property `portfolio-visible-count` có tăng không?
3. Slint có compile lại code không?

**Fix:**
Restart app: `cargo run`

---

## 📞 Next Steps

Sau khi test xong, có thể implement thêm:

### 1. Save portfolio data
Khi user nhập ticker/weight, save vào backend hoặc state

### 2. Search popup
Khi click 🔍, mở dialog để search ticker từ API

### 3. Validation
- Check duplicate tickers
- Convert ticker to uppercase
- Validate ticker format (3-4 chars)

### 4. Auto-suggest
Khi còn 1 row chưa nhập, suggest remaining weight

---

## ✨ Success Criteria

✅ Component hoạt động:
- "(More)" button adds 2 rows
- Maximum 20 rows
- Total calculation correct
- Validation works

✅ User experience tốt:
- Smooth interaction
- Clear feedback
- Intuitive UI

✅ Ready for next phase:
- Backend integration
- Search functionality
- Advanced validation

---

**Enjoy testing!** 🎉

Nếu gặp vấn đề, check console logs hoặc ping tôi!
