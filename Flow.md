## Tổng quan luồng hoạt động

`┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  PHASE 1 │───►│  PHASE 2 │───►│  PHASE 3 │───►│  PHASE 4 │───►│  PHASE 5 │
│  INPUT   │    │  FETCH   │    │  CALC    │    │  MONTE   │    │  STATS   │
│  PARAMS  │    │  DATA    │    │  PARAMS  │    │  CARLO   │    │  OUTPUT  │
└──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘`

---

## Phase 1: Thu thập Input từ User

`┌─────────────────────────────────────────────────────────────────────────┐
│                         PHASE 1: USER INPUT                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    PORTFOLIO ALLOCATION                          │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │  ┌───────────────────────────────────────────────────────────┐  │   │
│  │  │  Ticker      │  Weight (%)  │  Action                     │  │   │
│  │  ├──────────────┼──────────────┼─────────────────────────────┤  │   │
│  │  │  VNM         │  [  40  ]    │  [✕ Remove]                 │  │   │
│  │  │  FPT         │  [  35  ]    │  [✕ Remove]                 │  │   │
│  │  │  VCB         │  [  25  ]    │  [✕ Remove]                 │  │   │
│  │  ├──────────────┼──────────────┼─────────────────────────────┤  │   │
│  │  │  Total       │     100%     │  [+ Add Ticker]             │  │   │
│  │  └───────────────────────────────────────────────────────────┘  │   │
│  │                                                                 │   │
│  │  ⚠️ Validation: Total weight phải = 100%                        │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    BASIC PARAMETERS                              │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │  Initial Amount:            [  1,000,000,000  ] VND             │   │
│  │                                                                 │   │
│  │  Simulation Period:         [  5  ] Years                       │   │
│  │                                                                 │   │
│  │  Number of Simulations:     [  10,000  ]                        │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    MODEL PARAMETERS                              │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │  Simulation Model:          [ GBM ▼ ]                           │   │
│  │                             ┌──────────────────┐                │   │
│  │                             │ GBM              │                │   │
│  │                             │ Historical       │                │   │
│  │                             │ Jump Diffusion   │                │   │
│  │                             │ GARCH            │                │   │
│  │                             └──────────────────┘                │   │
│  │                                                                 │   │
│  │  Distribution:              [ Log-normal ▼ ]                    │   │
│  │  (Chỉ hiện khi Model = GBM)                                     │   │
│  │                                                                 │   │
│  │  Volatility Override:       ○ Auto (từ data)                    │   │
│  │                             ● Manual (nhập cho từng ticker)     │   │
│  │                                                                 │   │
│  │  ┌─────────────────────────────────────────────────────────┐    │   │
│  │  │  (Hiện khi chọn Manual)                                 │    │   │
│  │  │  Ticker    │  Volatility (%)                            │    │   │
│  │  │  ──────────┼─────────────────                           │    │   │
│  │  │  VNM       │  [  28.5  ]                                │    │   │
│  │  │  FPT       │  [  35.2  ]                                │    │   │
│  │  │  VCB       │  [  22.1  ]                                │    │   │
│  │  └─────────────────────────────────────────────────────────┘    │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    DISPLAY OPTIONS                               │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │  Confidence Interval:       [ 80% (10th-90th) ▼ ]               │   │
│  │                                                                 │   │
│  │  Risk-free Rate:            [  6.0  ]% (for Sharpe/Sortino)     │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│                        [ 🚀 Run Simulation ]                            │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│  OUTPUT:                                                                │
│                                                                         │
│  user_input = {                                                         │
│     portfolio: [                                                        │
│        { ticker: "VNM", weight: 0.40 },                                │
│        { ticker: "FPT", weight: 0.35 },                                │
│        { ticker: "VCB", weight: 0.25 }                                 │
│     ],                                                                  │
│     initial_amount: 1_000_000_000,                                     │
│     period_years: 5,                                                    │
│     num_simulations: 10_000,                                            │
│     model: "GBM",                                                       │
│     distribution: "log-normal",                                         │
│     volatility_mode: "auto" | { VNM: 0.285, FPT: 0.352, VCB: 0.221 },  │
│     confidence_interval: 0.80,                                          │
│     risk_free_rate: 0.06                                                │
│  }                                                                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘`

---

## Phase 2: Fetch dữ liệu lịch sử cho tất cả Tickers

`┌─────────────────────────────────────────────────────────────────────────┐
│                 PHASE 2: FETCH HISTORICAL DATA (MULTIPLE TICKERS)       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  INPUT: portfolio = [VNM (40%), FPT (35%), VCB (25%)]                  │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                 STEP 1: FETCH DATA FOR EACH TICKER               │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   FOR each ticker IN portfolio:                                 │   │
│  │   │                                                             │   │
│  │   │   // API Request                                            │   │
│  │   │   response = GET /historical-prices                         │   │
│  │   │                  ?ticker={ticker}                           │   │
│  │   │                  &period=5y                                 │   │
│  │   │                  &interval=daily                            │   │
│  │   │                                                             │   │
│  │   │   // Store data                                             │   │
│  │   │   prices[ticker] = response.close_prices                    │   │
│  │   │   current_price[ticker] = response.latest_price             │   │
│  │   │                                                             │   │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  │   Result:                                                       │   │
│  │   ┌─────────────────────────────────────────────────────────┐   │   │
│  │   │  prices["VNM"] = [126000, 125800, ..., 78500]           │   │   │
│  │   │  prices["FPT"] = [89000, 89500, ..., 125000]            │   │   │
│  │   │  prices["VCB"] = [85000, 84500, ..., 92000]             │   │   │
│  │   │                                                         │   │   │
│  │   │  current_price["VNM"] = 78,500 VND                      │   │   │
│  │   │  current_price["FPT"] = 125,000 VND                     │   │   │
│  │   │  current_price["VCB"] = 92,000 VND                      │   │   │
│  │   └─────────────────────────────────────────────────────────┘   │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                 STEP 2: CALCULATE DAILY RETURNS                  │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   FOR each ticker IN portfolio:                                 │   │
│  │      returns[ticker] = []                                       │   │
│  │      FOR i = 1 TO len(prices[ticker]) - 1:                      │   │
│  │         r = (prices[ticker][i] - prices[ticker][i-1])           │   │
│  │             / prices[ticker][i-1]                               │   │
│  │         returns[ticker].push(r)                                 │   │
│  │      END FOR                                                    │   │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  │   Result:                                                       │   │
│  │   ┌─────────────────────────────────────────────────────────┐   │   │
│  │   │  returns["VNM"] = [-0.0016, -0.0048, +0.0032, ...]      │   │   │
│  │   │  returns["FPT"] = [+0.0056, -0.0022, +0.0018, ...]      │   │   │
│  │   │  returns["VCB"] = [-0.0059, +0.0035, -0.0012, ...]      │   │   │
│  │   │                                                         │   │   │
│  │   │  Total: ~1,249 daily returns per ticker                 │   │   │
│  │   └─────────────────────────────────────────────────────────┘   │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                 STEP 3: ALIGN DATA (IMPORTANT!)                  │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   // Đảm bảo tất cả tickers có cùng ngày giao dịch              │   │
│  │   // (để tính correlation chính xác)                            │   │
│  │                                                                 │   │
│  │   common_dates = intersect(dates["VNM"], dates["FPT"], dates["VCB"]) │
│  │                                                                 │   │
│  │   FOR each ticker IN portfolio:                                 │   │
│  │      returns[ticker] = filter_by_dates(returns[ticker], common_dates) │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                 STEP 4: CALCULATE CORRELATION MATRIX             │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   // Tạo returns matrix                                         │   │
│  │   returns_matrix = [                                            │   │
│  │      returns["VNM"],   // Row 0                                 │   │
│  │      returns["FPT"],   // Row 1                                 │   │
│  │      returns["VCB"]    // Row 2                                 │   │
│  │   ]                                                             │   │
│  │                                                                 │   │
│  │   // Tính correlation matrix                                    │   │
│  │   correlation_matrix = calculate_correlation(returns_matrix)    │   │
│  │                                                                 │   │
│  │   Result:                                                       │   │
│  │   ┌─────────────────────────────────────────────────────────┐   │   │
│  │   │                                                         │   │   │
│  │   │   CORRELATION MATRIX (ρ)                                │   │   │
│  │   │                                                         │   │   │
│  │   │              VNM      FPT      VCB                      │   │   │
│  │   │            ┌──────┬──────┬──────┐                       │   │   │
│  │   │   VNM      │ 1.00 │ 0.45 │ 0.52 │                       │   │   │
│  │   │            ├──────┼──────┼──────┤                       │   │   │
│  │   │   FPT      │ 0.45 │ 1.00 │ 0.38 │                       │   │   │
│  │   │            ├──────┼──────┼──────┤                       │   │   │
│  │   │   VCB      │ 0.52 │ 0.38 │ 1.00 │                       │   │   │
│  │   │            └──────┴──────┴──────┘                       │   │   │
│  │   │                                                         │   │   │
│  │   │   Interpretation:                                       │   │   │
│  │   │   • VNM & FPT: Moderate positive correlation (0.45)     │   │   │
│  │   │   • VNM & VCB: Moderate positive correlation (0.52)     │   │   │
│  │   │   • FPT & VCB: Weak positive correlation (0.38)         │   │   │
│  │   │                                                         │   │   │
│  │   └─────────────────────────────────────────────────────────┘   │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│  OUTPUT:                                                                │
│                                                                         │
│  historical_data = {                                                    │
│     prices: {                                                           │
│        "VNM": [126000, 125800, ..., 78500],                            │
│        "FPT": [89000, 89500, ..., 125000],                             │
│        "VCB": [85000, 84500, ..., 92000]                               │
│     },                                                                  │
│     returns: {                                                          │
│        "VNM": [-0.0016, -0.0048, ...],                                 │
│        "FPT": [+0.0056, -0.0022, ...],                                 │
│        "VCB": [-0.0059, +0.0035, ...]                                  │
│     },                                                                  │
│     current_prices: {                                                   │
│        "VNM": 78500,                                                    │
│        "FPT": 125000,                                                   │
│        "VCB": 92000                                                     │
│     },                                                                  │
│     correlation_matrix: [                                               │
│        [1.00, 0.45, 0.52],                                             │
│        [0.45, 1.00, 0.38],                                             │
│        [0.52, 0.38, 1.00]                                              │
│     ],                                                                  │
│     ticker_order: ["VNM", "FPT", "VCB"]                                │
│  }                                                                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘`

---

## Phase 3: Tính toán Model Parameters

`┌─────────────────────────────────────────────────────────────────────────┐
│                   PHASE 3: CALCULATE MODEL PARAMETERS                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  INPUT:                                                                 │
│  • returns = { "VNM": [...], "FPT": [...], "VCB": [...] }              │
│  • weights = { "VNM": 0.40, "FPT": 0.35, "VCB": 0.25 }                 │
│  • correlation_matrix                                                   │
│  • model = "GBM"                                                        │
│  • volatility_mode = "auto"                                             │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │         STEP 1: CALCULATE INDIVIDUAL TICKER PARAMETERS           │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   FOR each ticker IN portfolio:                                 │   │
│  │   │                                                             │   │
│  │   │   // Daily Mean Return                                      │   │
│  │   │   μ_daily[ticker] = mean(returns[ticker])                   │   │
│  │   │                                                             │   │
│  │   │   // Annual Drift                                           │   │
│  │   │   μ[ticker] = μ_daily[ticker] × 252                         │   │
│  │   │                                                             │   │
│  │   │   // Daily Volatility                                       │   │
│  │   │   σ_daily[ticker] = std(returns[ticker])                    │   │
│  │   │                                                             │   │
│  │   │   // Annual Volatility                                      │   │
│  │   │   σ[ticker] = σ_daily[ticker] × √252                        │   │
│  │   │                                                             │   │
│  │   │   // Override if manual                                     │   │
│  │   │   IF volatility_mode != "auto":                             │   │
│  │   │      σ[ticker] = user_input.volatility[ticker]              │   │
│  │   │                                                             │   │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  │   Result:                                                       │   │
│  │   ┌─────────────────────────────────────────────────────────┐   │   │
│  │   │                                                         │   │   │
│  │   │   INDIVIDUAL PARAMETERS                                 │   │   │
│  │   │                                                         │   │   │
│  │   │   Ticker │ Weight │ μ (Drift) │ σ (Volatility)         │   │   │
│  │   │   ───────┼────────┼───────────┼─────────────────        │   │   │
│  │   │   VNM    │  40%   │  11.34%   │  28.57%                 │   │   │
│  │   │   FPT    │  35%   │  18.25%   │  35.20%                 │   │   │
│  │   │   VCB    │  25%   │   9.80%   │  22.10%                 │   │   │
│  │   │                                                         │   │   │
│  │   └─────────────────────────────────────────────────────────┘   │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │         STEP 2: CALCULATE PORTFOLIO EXPECTED RETURN              │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   // Portfolio Expected Return (weighted average)               │   │
│  │   μ_portfolio = Σ(wᵢ × μᵢ)                                      │   │
│  │                                                                 │   │
│  │              = w_VNM × μ_VNM + w_FPT × μ_FPT + w_VCB × μ_VCB    │   │
│  │              = 0.40 × 0.1134 + 0.35 × 0.1825 + 0.25 × 0.0980   │   │
│  │              = 0.0454 + 0.0639 + 0.0245                         │   │
│  │              = 0.1338 (13.38% / năm)                            │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │         STEP 3: CALCULATE PORTFOLIO VOLATILITY                   │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   // Portfolio Variance (có tính correlation)                   │   │
│  │   σ²_portfolio = Σᵢ Σⱼ (wᵢ × wⱼ × σᵢ × σⱼ × ρᵢⱼ)               │   │
│  │                                                                 │   │
│  │   // Expand formula cho 3 assets:                               │   │
│  │   σ²_p = w₁²σ₁² + w₂²σ₂² + w₃²σ₃²                              │   │
│  │        + 2×w₁×w₂×σ₁×σ₂×ρ₁₂                                     │   │
│  │        + 2×w₁×w₃×σ₁×σ₃×ρ₁₃                                     │   │
│  │        + 2×w₂×w₃×σ₂×σ₃×ρ₂₃                                     │   │
│  │                                                                 │   │
│  │   // Thay số:                                                   │   │
│  │   σ²_p = (0.40)²×(0.2857)² + (0.35)²×(0.3520)² + (0.25)²×(0.2210)² │
│  │        + 2×0.40×0.35×0.2857×0.3520×0.45                        │   │
│  │        + 2×0.40×0.25×0.2857×0.2210×0.52                        │   │
│  │        + 2×0.35×0.25×0.3520×0.2210×0.38                        │   │
│  │                                                                 │   │
│  │   σ²_p = 0.01306 + 0.01518 + 0.00305                           │   │
│  │        + 0.01267 + 0.00659 + 0.00518                           │   │
│  │        = 0.05573                                                │   │
│  │                                                                 │   │
│  │   // Portfolio Volatility                                       │   │
│  │   σ_portfolio = √0.05573 = 0.2361 (23.61% / năm)               │   │
│  │                                                                 │   │
│  │   Note: σ_portfolio (23.61%) < weighted average σ (28.3%)       │   │
│  │         → Diversification benefit!                              │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │         STEP 4: CHOLESKY DECOMPOSITION                           │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   // Để tạo correlated random numbers cho simulation            │   │
│  │   // Cần Cholesky decomposition của correlation matrix          │   │
│  │                                                                 │   │
│  │   // Correlation Matrix (ρ)                                     │   │
│  │   ρ = [ 1.00  0.45  0.52 ]                                      │   │
│  │       [ 0.45  1.00  0.38 ]                                      │   │
│  │       [ 0.52  0.38  1.00 ]                                      │   │
│  │                                                                 │   │
│  │   // Cholesky Decomposition: ρ = L × Lᵀ                         │   │
│  │   L = cholesky(ρ)                                               │   │
│  │                                                                 │   │
│  │   L = [ 1.0000  0.0000  0.0000 ]                                │   │
│  │       [ 0.4500  0.8930  0.0000 ]                                │   │
│  │       [ 0.5200  0.1630  0.8386 ]                                │   │
│  │                                                                 │   │
│  │   // Verification: L × Lᵀ = ρ ✓                                 │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │         STEP 5: MODEL-SPECIFIC PARAMETERS                        │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   SWITCH (model):                                               │   │
│  │                                                                 │   │
│  │   CASE "GBM":                                                   │   │
│  │      // Đã tính đủ: μ[], σ[], L (Cholesky)                      │   │
│  │                                                                 │   │
│  │   CASE "Historical Returns":                                    │   │
│  │      // Giữ nguyên returns[] cho bootstrap sampling             │   │
│  │                                                                 │   │
│  │   CASE "Jump Diffusion":                                        │   │
│  │      FOR each ticker:                                           │   │
│  │         threshold = 3 × σ_daily[ticker]                         │   │
│  │         jumps[ticker] = returns[ticker].filter(|r| > threshold) │   │
│  │         λ[ticker] = jumps.length / trading_days                 │   │
│  │         μ_j[ticker] = mean(jumps)                               │   │
│  │         σ_j[ticker] = std(jumps)                                │   │
│  │      END FOR                                                    │   │
│  │                                                                 │   │
│  │   CASE "GARCH":                                                 │   │
│  │      FOR each ticker:                                           │   │
│  │         {ω, α, β}[ticker] = fit_garch(returns[ticker])          │   │
│  │      END FOR                                                    │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│  OUTPUT:                                                                │
│                                                                         │
│  model_params = {                                                       │
│     model: "GBM",                                                       │
│     distribution: "log-normal",                                         │
│     individual: {                                                       │
│        "VNM": { μ: 0.1134, σ: 0.2857 },                                │
│        "FPT": { μ: 0.1825, σ: 0.3520 },                                │
│        "VCB": { μ: 0.0980, σ: 0.2210 }                                 │
│     },                                                                  │
│     portfolio: {                                                        │
│        μ: 0.1338,                                                       │
│        σ: 0.2361                                                        │
│     },                                                                  │
│     cholesky_matrix: L,                                                 │
│     correlation_matrix: ρ                                               │
│  }                                                                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘`

---

## Phase 4: Chạy Monte Carlo Simulation

`┌─────────────────────────────────────────────────────────────────────────┐
│              PHASE 4: MONTE CARLO ENGINE (MULTIPLE TICKERS)             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  INPUT:                                                                 │
│  • current_prices = { VNM: 78500, FPT: 125000, VCB: 92000 }            │
│  • weights = { VNM: 0.40, FPT: 0.35, VCB: 0.25 }                       │
│  • initial_amount = 1,000,000,000 VND                                  │
│  • T = 5 years                                                          │
│  • N = 10,000 simulations                                               │
│  • model_params (μ, σ for each ticker, Cholesky matrix L)              │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    INITIALIZATION                                │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   // Tính số cổ phiếu cho mỗi ticker                            │   │
│  │   allocated_amount = {}                                         │   │
│  │   shares = {}                                                   │   │
│  │                                                                 │   │
│  │   FOR each ticker IN portfolio:                                 │   │
│  │      allocated_amount[ticker] = initial_amount × weight[ticker] │   │
│  │      shares[ticker] = allocated_amount[ticker] / current_price[ticker] │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  │   Result:                                                       │   │
│  │   ┌─────────────────────────────────────────────────────────┐   │   │
│  │   │  Ticker │ Weight │ Allocated      │ Price   │ Shares   │   │   │
│  │   │  ───────┼────────┼────────────────┼─────────┼──────────│   │   │
│  │   │  VNM    │  40%   │ 400,000,000    │ 78,500  │ 5,096    │   │   │
│  │   │  FPT    │  35%   │ 350,000,000    │ 125,000 │ 2,800    │   │   │
│  │   │  VCB    │  25%   │ 250,000,000    │ 92,000  │ 2,717    │   │   │
│  │   │  ───────┼────────┼────────────────┼─────────┼──────────│   │   │
│  │   │  Total  │ 100%   │ 1,000,000,000  │         │          │   │   │
│  │   └─────────────────────────────────────────────────────────┘   │   │
│  │                                                                 │   │
│  │   // Tính số bước thời gian                                     │   │
│  │   dt = 1 / 252                                                  │   │
│  │   total_steps = T × 252 = 1,260 trading days                    │   │
│  │                                                                 │   │
│  │   // Khởi tạo arrays                                            │   │
│  │   price_paths[ticker][N][total_steps + 1]    // Giá từng ticker │   │
│  │   portfolio_paths[N][total_steps + 1]        // Giá trị portfolio │
│  │   ending_values[N]                            // Giá trị cuối   │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    SIMULATION LOOP                               │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │   FOR sim = 1 TO 10,000:                                        │   │
│  │   │                                                             │   │
│  │   │   // Initialize prices at t=0                               │   │
│  │   │   FOR each ticker IN portfolio:                             │   │
│  │   │      price_paths[ticker][sim][0] = current_price[ticker]    │   │
│  │   │   END FOR                                                   │   │
│  │   │                                                             │   │
│  │   │   // Calculate initial portfolio value                      │   │
│  │   │   portfolio_paths[sim][0] = initial_amount                  │   │
│  │   │                                                             │   │
│  │   │   FOR t = 1 TO 1,260:                                       │   │
│  │   │   │                                                         │   │
│  │   │   │   ┌─────────────────────────────────────────────────┐   │   │
│  │   │   │   │  STEP A: GENERATE CORRELATED RANDOM NUMBERS     │   │   │
│  │   │   │   ├─────────────────────────────────────────────────┤   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   │  // Generate independent standard normal        │   │   │
│  │   │   │   │  Z_independent = [                              │   │   │
│  │   │   │   │     random_normal(0, 1),  // for VNM            │   │   │
│  │   │   │   │     random_normal(0, 1),  // for FPT            │   │   │
│  │   │   │   │     random_normal(0, 1)   // for VCB            │   │   │
│  │   │   │   │  ]                                              │   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   │  // Transform to correlated using Cholesky      │   │   │
│  │   │   │   │  Z_correlated = L × Z_independent               │   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   │  // Result: Z_correlated có correlation = ρ     │   │   │
│  │   │   │   │  Z["VNM"] = Z_correlated[0]                     │   │   │
│  │   │   │   │  Z["FPT"] = Z_correlated[1]                     │   │   │
│  │   │   │   │  Z["VCB"] = Z_correlated[2]                     │   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   └─────────────────────────────────────────────────┘   │   │
│  │   │   │                                                         │   │
│  │   │   │   ┌─────────────────────────────────────────────────┐   │   │
│  │   │   │   │  STEP B: SIMULATE PRICE FOR EACH TICKER         │   │   │
│  │   │   │   ├─────────────────────────────────────────────────┤   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   │  FOR each ticker IN portfolio:                  │   │   │
│  │   │   │   │  │                                              │   │   │
│  │   │   │   │  │  S_prev = price_paths[ticker][sim][t-1]      │   │   │
│  │   │   │   │  │  μ_i = model_params.individual[ticker].μ     │   │   │
│  │   │   │   │  │  σ_i = model_params.individual[ticker].σ     │   │   │
│  │   │   │   │  │  Z_i = Z[ticker]                             │   │   │
│  │   │   │   │  │                                              │   │   │
│  │   │   │   │  │  SWITCH (model):                             │   │   │
│  │   │   │   │  │                                              │   │   │
│  │   │   │   │  │  CASE "GBM" + "Log-normal":                  │   │   │
│  │   │   │   │  │     drift = (μ_i - σ_i²/2) × dt              │   │   │
│  │   │   │   │  │     diffusion = σ_i × √dt × Z_i              │   │   │
│  │   │   │   │  │     S_new = S_prev × exp(drift + diffusion)  │   │   │
│  │   │   │   │  │                                              │   │   │
│  │   │   │   │  │  CASE "GBM" + "Normal":                      │   │   │
│  │   │   │   │  │     S_new = S_prev × (1 + μ_i×dt + σ_i×√dt×Z_i) │  │
│  │   │   │   │  │                                              │   │   │
│  │   │   │   │  │  CASE "Historical Returns":                  │   │   │
│  │   │   │   │  │     // Chọn cùng ngày cho tất cả tickers     │   │   │
│  │   │   │   │  │     // để maintain correlation               │   │   │
│  │   │   │   │  │     random_day = random_int(0, num_days-1)   │   │   │
│  │   │   │   │  │     r = returns[ticker][random_day]          │   │   │
│  │   │   │   │  │     S_new = S_prev × (1 + r)                 │   │   │
│  │   │   │   │  │                                              │   │   │
│  │   │   │   │  │  CASE "Jump Diffusion":                      │   │   │
│  │   │   │   │  │     base = (μ_i - σ_i²/2)×dt + σ_i×√dt×Z_i   │   │   │
│  │   │   │   │  │     jump = random() < λ[ticker]×dt           │   │   │
│  │   │   │   │  │            ? random_normal(μ_j, σ_j) : 0     │   │   │
│  │   │   │   │  │     S_new = S_prev × exp(base + jump)        │   │   │
│  │   │   │   │  │                                              │   │   │
│  │   │   │   │  │  price_paths[ticker][sim][t] = S_new         │   │   │
│  │   │   │   │  │                                              │   │   │
│  │   │   │   │  END FOR                                        │   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   └─────────────────────────────────────────────────┘   │   │
│  │   │   │                                                         │   │
│  │   │   │   ┌─────────────────────────────────────────────────┐   │   │
│  │   │   │   │  STEP C: CALCULATE PORTFOLIO VALUE              │   │   │
│  │   │   │   ├─────────────────────────────────────────────────┤   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   │  portfolio_value = 0                            │   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   │  FOR each ticker IN portfolio:                  │   │   │
│  │   │   │   │     ticker_value = shares[ticker]               │   │   │
│  │   │   │   │                    × price_paths[ticker][sim][t]│   │   │
│  │   │   │   │     portfolio_value += ticker_value             │   │   │
│  │   │   │   │  END FOR                                        │   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   │  portfolio_paths[sim][t] = portfolio_value      │   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   │  // Example at t=1:                             │   │   │
│  │   │   │   │  // VNM: 5,096 × 78,800 = 401,564,800           │   │   │
│  │   │   │   │  // FPT: 2,800 × 125,500 = 351,400,000          │   │   │
│  │   │   │   │  // VCB: 2,717 × 92,300 = 250,759,100           │   │   │
│  │   │   │   │  // Total: 1,003,723,900 VND                    │   │   │
│  │   │   │   │                                                 │   │   │
│  │   │   │   └─────────────────────────────────────────────────┘   │   │
│  │   │   │                                                         │   │
│  │   │   END FOR (t loop)                                          │   │
│  │   │                                                             │   │
│  │   │   // Store ending value                                     │   │
│  │   │   ending_values[sim] = portfolio_paths[sim][1260]           │   │
│  │   │                                                             │   │
│  │   END FOR (sim loop)                                            │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│  OUTPUT:                                                                │
│                                                                         │
│  simulation_results = {                                                 │
│     price_paths: {                                                      │
│        "VNM": [10,000][1,261],                                         │
│        "FPT": [10,000][1,261],                                         │
│        "VCB": [10,000][1,261]                                          │
│     },                                                                  │
│     portfolio_paths: [10,000][1,261],                                  │
│     ending_values: [10,000],                                            │
│     shares: { "VNM": 5096, "FPT": 2800, "VCB": 2717 },                 │
│     initial_amount: 1,000,000,000                                      │
│  }                                                                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘`

---

## Phase 5: Tính toán Statistics & Generate Output

`┌─────────────────────────────────────────────────────────────────────────┐
│              PHASE 5: STATISTICAL ANALYSIS & OUTPUT                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  INPUT:                                                                 │
│  • portfolio_paths[10,000][1,261]                                      │
│  • ending_values[10,000]                                                │
│  • initial_amount = 1,000,000,000 VND                                  │
│  • risk_free_rate = 0.06                                                │
│  • confidence_interval = 0.80                                           │
│  • T = 5 years                                                          │
│                                                                         │
│ ═══════════════════════════════════════════════════════════════════════ │
│  STEP 1: TÍNH METRICS CHO MỖI SIMULATION                                │
│ ═══════════════════════════════════════════════════════════════════════ │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                                                                 │   │
│  │   FOR sim = 1 TO 10,000:                                        │   │
│  │   │                                                             │   │
│  │   │   path = portfolio_paths[sim]                               │   │
│  │   │                                                             │   │
│  │   │   // 1. Portfolio End Value                                 │   │
│  │   │   end_values[sim] = ending_values[sim]                      │   │
│  │   │                                                             │   │
│  │   │   // 2. Total Return (%)                                    │   │
│  │   │   total_returns[sim] = (end_values[sim] - initial) / initial│   │
│  │   │                                                             │   │
│  │   │   // 3. Annualized Return (CAGR)                            │   │
│  │   │   cagr[sim] = (end_values[sim] / initial)^(1/T) - 1         │   │
│  │   │                                                             │   │
│  │   │   // 4. Annualized Volatility                               │   │
│  │   │   daily_returns = []                                        │   │
│  │   │   FOR t = 1 TO 1260:                                        │   │
│  │   │      r = (path[t] - path[t-1]) / path[t-1]                  │   │
│  │   │      daily_returns.push(r)                                  │   │
│  │   │   END FOR                                                   │   │
│  │   │   volatility[sim] = std(daily_returns) × √252               │   │
│  │   │                                                             │   │
│  │   │   // 5. Sharpe Ratio                                        │   │
│  │   │   sharpe[sim] = (cagr[sim] - Rf) / volatility[sim]          │   │
│  │   │                                                             │   │
│  │   │   // 6. Sortino Ratio                                       │   │
│  │   │   negative_returns = daily_returns.filter(r => r < 0)       │   │
│  │   │   downside_std = std(negative_returns) × √252               │   │
│  │   │   sortino[sim] = (cagr[sim] - Rf) / downside_std            │   │
│  │   │                                                             │   │
│  │   │   // 7. Maximum Drawdown                                    │   │
│  │   │   peak = path[0]                                            │   │
│  │   │   max_dd[sim] = 0                                           │   │
│  │   │   FOR t = 1 TO 1260:                                        │   │
│  │   │      IF path[t] > peak: peak = path[t]                      │   │
│  │   │      drawdown = (peak - path[t]) / peak                     │   │
│  │   │      IF drawdown > max_dd[sim]: max_dd[sim] = drawdown      │   │
│  │   │   END FOR                                                   │   │
│  │   │                                                             │   │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│ ═══════════════════════════════════════════════════════════════════════ │
│  STEP 2: TÍNH PERCENTILES                                               │
│ ═══════════════════════════════════════════════════════════════════════ │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                                                                 │   │
│  │   metrics = [end_values, total_returns, cagr, volatility,       │   │
│  │              sharpe, sortino, max_dd]                           │   │
│  │                                                                 │   │
│  │   FOR each metric IN metrics:                                   │   │
│  │      sorted = sort(metric)                                      │   │
│  │      percentiles[metric] = {                                    │   │
│  │         p10: sorted[1000],                                      │   │
│  │         p25: sorted[2500],                                      │   │
│  │         p50: sorted[5000],                                      │   │
│  │         p75: sorted[7500],                                      │   │
│  │         p90: sorted[9000]                                       │   │
│  │      }                                                          │   │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│ ═══════════════════════════════════════════════════════════════════════ │
│  STEP 3: TÍNH PROBABILITIES                                             │
│ ═══════════════════════════════════════════════════════════════════════ │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                                                                 │   │
│  │   p_gain = count(total_returns > 0) / N                         │   │
│  │                                                                 │   │
│  │   bank_return_5y = (1 + Rf)^T - 1                               │   │
│  │   p_beat_bank = count(total_returns > bank_return_5y) / N       │   │
│  │                                                                 │   │
│  │   p_double = count(total_returns > 1.0) / N                     │   │
│  │                                                                 │   │
│  │   p_lose_25 = count(total_returns < -0.25) / N                  │   │
│  │                                                                 │   │
│  │   p_lose_50 = count(total_returns < -0.50) / N                  │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│ ═══════════════════════════════════════════════════════════════════════ │
│  STEP 4: TÍNH YEARLY PERCENTILES (cho Line Chart)                       │
│ ═══════════════════════════════════════════════════════════════════════ │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                                                                 │   │
│  │   yearly_percentiles = {                                        │   │
│  │      years: [0, 1, 2, 3, 4, 5],                                 │   │
│  │      p10: [], p25: [], p50: [], p75: [], p90: []                │   │
│  │   }                                                             │   │
│  │                                                                 │   │
│  │   FOR year = 0 TO T:                                            │   │
│  │      t = year × 252                                             │   │
│  │      values_at_t = [portfolio_paths[sim][t] for sim in 1..N]    │   │
│  │                                                                 │   │
│  │      sorted = sort(values_at_t)                                 │   │
│  │      yearly_percentiles.p10[year] = sorted[1000]                │   │
│  │      yearly_percentiles.p25[year] = sorted[2500]                │   │
│  │      yearly_percentiles.p50[year] = sorted[5000]                │   │
│  │      yearly_percentiles.p75[year] = sorted[7500]                │   │
│  │      yearly_percentiles.p90[year] = sorted[9000]                │   │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│ ═══════════════════════════════════════════════════════════════════════ │
│  STEP 5: TÍNH HISTOGRAM DATA                                            │
│ ═══════════════════════════════════════════════════════════════════════ │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                                                                 │   │
│  │   // Lọc 95% kết quả                                            │   │
│  │   sorted_end_values = sort(end_values)                          │   │
│  │   p2_5 = sorted_end_values[250]                                 │   │
│  │   p97_5 = sorted_end_values[9750]                               │   │
│  │                                                                 │   │
│  │   filtered_values = end_values.filter(v => v >= p2_5 && v <= p97_5) │
│  │                                                                 │   │
│  │   // Tạo bins                                                   │   │
│  │   num_bins = 20                                                 │   │
│  │   bin_width = (p97_5 - p2_5) / num_bins                         │   │
│  │                                                                 │   │
│  │   histogram_data = []                                           │   │
│  │   FOR i = 0 TO num_bins - 1:                                    │   │
│  │      bin_start = p2_5 + i × bin_width                           │   │
│  │      bin_end = bin_start + bin_width                            │   │
│  │      count = filtered_values.filter(v => v >= bin_start && v < bin_end).length │
│  │      histogram_data.push({                                      │   │
│  │         range: [bin_start, bin_end],                            │   │
│  │         label: format_currency(bin_start),                      │   │
│  │         frequency: count                                        │   │
│  │      })                                                         │   │
│  │   END FOR                                                       │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│ ═══════════════════════════════════════════════════════════════════════ │
│  STEP 6: GENERATE OUTPUTS                                               │
│ ═══════════════════════════════════════════════════════════════════════ │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │           OUTPUT 1: PORTFOLIO BALANCE LINE CHART                 │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │  Title: "Simulated Portfolio Balances"                          │   │
│  │                                                                 │   │
│  │  Portfolio Balance (VND)                                        │   │
│  │      │                                                          │   │
│  │  4B  ┤                                          ─── 90th        │   │
│  │      │                                    ─────────             │   │
│  │  3B  ┤                              ─────────────               │   │
│  │      │                        ─────────────────── 75th          │   │
│  │  2B  ┤                  ─────────────────────────               │   │
│  │      │            ─────────────────────────────── 50th          │   │
│  │ 1.5B ┤      ─────────────────────────────────────               │   │
│  │      │ ─────────────────────────────────────────── 25th         │   │
│  │  1B  ┼───────────────────────────────────────────               │   │
│  │      │ ─────────────────────────────────────────── 10th         │   │
│  │ 500M ┤                                                          │   │
│  │      └────┬────┬────┬────┬────┬────                             │   │
│  │           0    1    2    3    4    5   Year                     │   │
│  │                                                                 │   │
│  │  Legend:                                                        │   │
│  │  ── 10th Percentile  ── 25th Percentile  ── 50th Percentile    │   │
│  │  ── 75th Percentile  ── 90th Percentile                        │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │         OUTPUT 2: PORTFOLIO END BALANCE HISTOGRAM                │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │  Title: "Portfolio End Balance Histogram (95% of results)"      │   │
│  │                                                                 │   │
│  │  Frequency                                                      │   │
│  │      │                                                          │   │
│  │ 2500 ┤  ████                                                    │   │
│  │      │  ████ ████                                               │   │
│  │ 2000 ┤  ████ ████                                               │   │
│  │      │  ████ ████ ████                                          │   │
│  │ 1500 ┤  ████ ████ ████ ████                                     │   │
│  │      │  ████ ████ ████ ████ ████                                │   │
│  │ 1000 ┤  ████ ████ ████ ████ ████ ████                           │   │
│  │      │  ████ ████ ████ ████ ████ ████ ████                      │   │
│  │  500 ┤  ████ ████ ████ ████ ████ ████ ████ ████ ████            │   │
│  │      │  ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████  │   │
│  │    0 ┼──────────────────────────────────────────────────────    │   │
│  │        500M  1B  1.5B  2B  2.5B  3B  3.5B  4B  4.5B  5B        │   │
│  │                 Ending Portfolio Value (VND)                    │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                 OUTPUT 3: PERFORMANCE SUMMARY                    │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │                                                                 │   │
│  │  PORTFOLIO ALLOCATION                                           │   │
│  │  ══════════════════════════════════════════════════════════════ │   │
│  │  Ticker    │ Weight │ Allocated      │ Shares  │ μ      │ σ     │   │
│  │  ──────────┼────────┼────────────────┼─────────┼────────┼────── │   │
│  │  VNM       │  40%   │ 400,000,000    │ 5,096   │ 11.34% │ 28.57%│   │
│  │  FPT       │  35%   │ 350,000,000    │ 2,800   │ 18.25% │ 35.20%│   │
│  │  VCB       │  25%   │ 250,000,000    │ 2,717   │  9.80% │ 22.10%│   │
│  │  ──────────┼────────┼────────────────┼─────────┼────────┼────── │   │
│  │  Portfolio │ 100%   │ 1,000,000,000  │         │ 13.38% │ 23.61%│   │
│  │                                                                 │   │
│  │  CORRELATION MATRIX                                             │   │
│  │  ══════════════════════════════════════════════════════════════ │   │
│  │              VNM      FPT      VCB                              │   │
│  │  VNM         1.00     0.45     0.52                             │   │
│  │  FPT         0.45     1.00     0.38                             │   │
│  │  VCB         0.52     0.38     1.00                             │   │
│  │                                                                 │   │
│  │  SIMULATION PARAMETERS                                          │   │
│  │  ══════════════════════════════════════════════════════════════ │   │
│  │  Initial Investment:        1,000,000,000 VND                   │   │
│  │  Simulation Period:         5 years                             │   │
│  │  Number of Simulations:     10,000                              │   │
│  │  Model:                     GBM (Log-normal)                    │   │
│  │  Risk-free Rate:            6.00%                               │   │
│  │                                                                 │   │
│  │                    10th      25th      50th      75th      90th │   │
│  │                    ────      ────      ────      ────      ──── │   │
│  │  RETURN METRICS                                                 │   │
│  │  ══════════════════════════════════════════════════════════════ │   │
│  │  Portfolio End    685M    982M     1.42B    1.98B     2.85B     │   │
│  │  Value (VND)                                                    │   │
│  │                                                                 │   │
│  │  Total Return    -31.5%   -1.8%    +42.0%   +98.5%   +185.0%    │   │
│  │                                                                 │   │
│  │  Annualized       -7.3%   -0.4%     +7.3%   +14.7%    +23.3%    │   │
│  │  Return (CAGR)                                                  │   │
│  │                                                                 │   │
│  │  Annualized       21.5%   22.3%     23.6%    24.8%     26.1%    │   │
│  │  Volatility                                                     │   │
│  │                                                                 │   │
│  │  RISK-ADJUSTED METRICS                                          │   │
│  │  ══════════════════════════════════════════════════════════════ │   │
│  │  Sharpe Ratio     -0.62   -0.29     +0.06    +0.35     +0.66    │   │
│  │                                                                 │   │
│  │  Sortino Ratio    -0.52   -0.22     +0.09    +0.52     +1.02    │   │
│  │                                                                 │   │
│  │  Maximum         -58.2%  -45.1%    -32.5%   -23.8%    -16.2%    │   │
│  │  Drawdown                                                       │   │
│  │                                                                 │   │
│  │  PROBABILITY ANALYSIS                                           │   │
│  │  ══════════════════════════════════════════════════════════════ │   │
│  │  P(Gain > 0%):                      75.8%                       │   │
│  │  P(Beat Bank Rate 6%/yr):           62.4%                       │   │
│  │  P(Double Initial):                 22.1%                       │   │
│  │  P(Lose > 25%):                     12.3%                       │   │
│  │  P(Lose > 50%):                      4.1%                       │   │
│  │                                                                 │   │
│  │  CONFIDENCE INTERVAL (80%)                                      │   │
│  │  ══════════════════════════════════════════════════════════════ │   │
│  │  Portfolio Value:   685,000,000 - 2,850,000,000 VND             │   │
│  │  Total Return:      -31.5% to +185.0%                           │   │
│  │  Interpretation:    80% khả năng portfolio sẽ nằm trong         │   │
│  │                     khoảng này sau 5 năm                        │   │
│  │                                                                 │   │
│  │  DIVERSIFICATION BENEFIT                                        │   │
│  │  ══════════════════════════════════════════════════════════════ │   │
│  │  Weighted Avg Volatility:   28.3%                               │   │
│  │  Portfolio Volatility:      23.6%                               │   │
│  │  Volatility Reduction:      -4.7% (diversification benefit)     │   │
│  │                                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘`

---

## Complete Flow Diagram

`┌─────────────────────────────────────────────────────────────────────────┐
│                        COMPLETE FLOW DIAGRAM                            │
│                      (MULTIPLE TICKERS VERSION)                         │
└─────────────────────────────────────────────────────────────────────────┘

                              ┌─────────────┐
                              │   START     │
                              └──────┬──────┘
                                     ▼
                    ┌────────────────────────────────┐
                    │        PHASE 1: INPUT          │
                    │  • Tickers + Weights           │
                    │  • Initial Amount              │
                    │  • Period (Years)              │
                    │  • Number of Simulations       │
                    │  • Model & Distribution        │
                    │  • Volatility (Auto/Manual)    │
                    │  • Confidence Interval         │
                    │  • Risk-free Rate              │
                    └────────────────┬───────────────┘
                                     ▼
                    ┌────────────────────────────────┐
                    │      PHASE 2: FETCH DATA       │
                    │  • Fetch prices for ALL tickers│
                    │  • Calculate returns           │
                    │  • Align data (common dates)   │
                    │  • Calculate Correlation Matrix│
                    └────────────────┬───────────────┘
                                     ▼
                    ┌────────────────────────────────┐
                    │    PHASE 3: CALC PARAMETERS    │
                    │  • Individual μ, σ per ticker  │
                    │  • Portfolio μ, σ              │
                    │  • Cholesky Decomposition      │
                    │  • Model-specific params       │
                    └────────────────┬───────────────┘
                                     ▼
                    ┌────────────────────────────────┐
                    │   PHASE 4: MONTE CARLO ENGINE  │
                    │                                │
                    │  FOR sim = 1 TO N:             │
                    │    FOR t = 1 TO steps:         │
                    │      Generate correlated Z     │
                    │      Simulate each ticker      │
                    │      Calculate portfolio value │
                    │    END                         │
                    │  END                           │
                    └────────────────┬───────────────┘
                                     ▼
          ┌──────────────────────────────────────────────────────────┐
          │                PHASE 5: STATISTICS & OUTPUT              │
          ├──────────────────────────────────────────────────────────┤
          │                                                          │
          │  STEP 1: Calculate Metrics (per simulation)              │
          │  STEP 2: Calculate Percentiles                           │
          │  STEP 3: Calculate Probabilities                         │
          │  STEP 4: Calculate Yearly Percentiles                    │
          │  STEP 5: Calculate Histogram Data                        │
          │  STEP 6: Generate Outputs                                │
          │                                                          │
          │  ┌────────────────────────────────────────────────────┐  │
          │  │                                                    │  │
          │  │  ┌─────────────────┐  ┌─────────────────┐          │  │
          │  │  │  LINE CHART     │  │   HISTOGRAM     │          │  │
          │  │  │  Portfolio      │  │   End Balance   │          │  │
          │  │  │  Balance        │  │   Distribution  │          │  │
          │  │  └─────────────────┘  └─────────────────┘          │  │
          │  │                                                    │  │
          │  │  ┌─────────────────────────────────────────────┐   │  │
          │  │  │           PERFORMANCE SUMMARY               │   │  │
          │  │  │  • Portfolio Allocation                     │   │  │
          │  │  │  • Correlation Matrix                       │   │  │
          │  │  │  • Return Metrics (by percentile)           │   │  │
          │  │  │  • Risk-Adjusted Metrics                    │   │  │
          │  │  │  • Probability Analysis                     │   │  │
          │  │  │  • Diversification Benefit                  │   │  │
          │  │  └─────────────────────────────────────────────┘   │  │
          │  │                                                    │  │
          │  └────────────────────────────────────────────────────┘  │
          │                                                          │
          └──────────────────────────────────────────────────────────┘
                                     │
                                     ▼
                              ┌─────────────┐
                              │     END     │
                              └─────────────┘`