# Sprint Plan - Monte Carlo Simulation

## Tổng quan
Sprint này tập trung vào việc hoàn thiện chức năng Monte Carlo Simulation cho danh mục đầu tư nhiều mã cổ phiếu (Multiple Tickers) theo đúng luồng được mô tả trong Flow.md.

---

## Phase 1: User Input Interface (UI)

### 1.1 Portfolio Allocation Table Component
- [ ] Tạo component PortfolioAllocationTable với khả năng:
  - Hiển thị danh sách các ticker với weight tương ứng
  - Cho phép thêm ticker mới bằng nút "Add Ticker"
  - Cho phép xóa ticker bằng nút "Remove"
  - Input field để nhập % weight cho mỗi ticker
  - Hiển thị tổng weight và tự động tính toán
- [ ] Implement validation logic:
  - Kiểm tra tổng weight phải = 100%
  - Hiển thị warning nếu tổng weight không hợp lệ
  - Kiểm tra ticker trùng lặp
  - Kiểm tra weight phải > 0

### 1.2 Basic Parameters Section
- [ ] Tạo input field cho Initial Amount (số tiền ban đầu):
  - Hỗ trợ format VND với dấu phẩy
  - Validation: phải > 0
- [ ] Tạo input field cho Simulation Period (số năm):
  - Default: 5 years
  - Validation: phải > 0 và <= 20
- [ ] Tạo input field cho Number of Simulations:
  - Default: 10,000
  - Validation: phải >= 1,000 và <= 100,000

### 1.3 Model Parameters Section
- [ ] Tạo dropdown "Simulation Model" với các options:
  - GBM (Geometric Brownian Motion)
  - Historical Returns
  - Jump Diffusion
  - GARCH
- [ ] Tạo dropdown "Distribution" (chỉ hiện khi Model = GBM):
  - Log-normal
  - Normal
- [ ] Tạo radio buttons cho Volatility Override:
  - Auto (tính từ dữ liệu lịch sử)
  - Manual (nhập cho từng ticker)
- [ ] Tạo Volatility Input Table (chỉ hiện khi chọn Manual):
  - Hiển thị danh sách tickers từ Portfolio Allocation
  - Input field cho volatility % của mỗi ticker
  - Auto-update khi thay đổi portfolio

### 1.4 Display Options Section
- [ ] Tạo dropdown "Confidence Interval" với các options:
  - 80% (10th-90th)
  - 90% (5th-95th)
  - 95% (2.5th-97.5th)
- [ ] Tạo input field cho Risk-free Rate:
  - Default: 6.0%
  - Dùng để tính Sharpe Ratio và Sortino Ratio

### 1.5 Run Button & State Management
- [ ] Tạo button "Run Simulation" với:
  - Disable khi validation chưa pass
  - Loading state khi đang chạy simulation
  - Error handling và hiển thị thông báo lỗi
- [ ] Setup state management để lưu trữ:
  - User input parameters
  - Validation status
  - Simulation running status

---

## Phase 2: Data Fetching & Processing (Backend/Logic)

### 2.1 API Integration
- [ ] Implement function fetch dữ liệu lịch sử cho một ticker:
  - API endpoint: GET /historical-prices
  - Parameters: ticker, period, interval
  - Xử lý response và parse data
- [ ] Implement function fetch dữ liệu cho nhiều tickers:
  - Gọi API tuần tự hoặc song song cho từng ticker
  - Xử lý errors (ticker không tồn tại, network error)
  - Lưu trữ kết quả: prices[], current_price[]

### 2.2 Returns Calculation
- [ ] Implement function tính daily returns:
  - Formula: r(t) = (P(t) - P(t-1)) / P(t-1)
  - Apply cho tất cả tickers
  - Output: returns[ticker] = array of returns

### 2.3 Data Alignment
- [ ] Implement function align data theo common dates:
  - Tìm intersection của dates cho tất cả tickers
  - Filter returns[] để chỉ giữ common dates
  - Đảm bảo tất cả tickers có cùng số lượng data points

### 2.4 Correlation Matrix Calculation
- [ ] Implement function tính correlation matrix:
  - Input: returns_matrix (mỗi row là returns của 1 ticker)
  - Output: correlation matrix (N x N)
  - Sử dụng Pearson correlation coefficient
- [ ] Verify correlation matrix:
  - Diagonal elements = 1.0
  - Matrix phải symmetric
  - Values trong range [-1, 1]

---

## Phase 3: Model Parameters Calculation (Backend/Logic)

### 3.1 Individual Ticker Parameters
- [ ] Implement function tính parameters cho từng ticker:
  - Daily mean return: μ_daily
  - Annual drift: μ = μ_daily × 252
  - Daily volatility: σ_daily = std(returns)
  - Annual volatility: σ = σ_daily × √252
- [ ] Implement override logic:
  - Nếu volatility_mode = "manual", sử dụng user input
  - Nếu volatility_mode = "auto", sử dụng calculated σ

### 3.2 Portfolio-Level Parameters
- [ ] Implement function tính portfolio expected return:
  - μ_portfolio = Σ(w_i × μ_i)
  - Weighted average của individual returns
- [ ] Implement function tính portfolio volatility:
  - σ²_portfolio = Σ_i Σ_j (w_i × w_j × σ_i × σ_j × ρ_ij)
  - Expand formula cho N assets
  - σ_portfolio = √(σ²_portfolio)

### 3.3 Cholesky Decomposition
- [ ] Implement function Cholesky decomposition:
  - Input: correlation matrix ρ
  - Output: lower triangular matrix L
  - Verify: L × L^T = ρ
- [ ] Handle edge cases:
  - Matrix không positive definite
  - Numerical stability issues

### 3.4 Model-Specific Parameters
- [ ] Implement parameter calculation cho GBM:
  - Đã có: μ[], σ[], L
- [ ] Implement parameter calculation cho Historical Returns:
  - Giữ nguyên returns[] cho bootstrap sampling
- [ ] Implement parameter calculation cho Jump Diffusion:
  - Detect jumps: threshold = 3 × σ_daily
  - Calculate λ (jump frequency)
  - Calculate μ_j, σ_j (jump mean, std)
- [ ] Implement parameter calculation cho GARCH:
  - Fit GARCH(1,1) model
  - Estimate parameters: ω, α, β

---

## Phase 4: Monte Carlo Engine (Backend/Logic)

### 4.1 Initialization
- [ ] Implement function tính số cổ phiếu cho mỗi ticker:
  - allocated_amount[ticker] = initial_amount × weight[ticker]
  - shares[ticker] = allocated_amount[ticker] / current_price[ticker]
- [ ] Setup simulation arrays:
  - price_paths[ticker][N][steps+1]
  - portfolio_paths[N][steps+1]
  - ending_values[N]
- [ ] Calculate simulation constants:
  - dt = 1/252
  - total_steps = years × 252

### 4.2 Correlated Random Number Generation
- [ ] Implement function generate correlated random numbers:
  - Generate Z_independent (N independent standard normals)
  - Transform: Z_correlated = L × Z_independent
  - Verify correlation của Z_correlated = ρ

### 4.3 Price Simulation for Each Model
- [ ] Implement GBM simulation (Log-normal):
  - Formula: S_new = S_prev × exp((μ - σ²/2)×dt + σ×√dt×Z)
- [ ] Implement GBM simulation (Normal):
  - Formula: S_new = S_prev × (1 + μ×dt + σ×√dt×Z)
- [ ] Implement Historical Returns simulation:
  - Randomly sample từ returns[] (cùng ngày cho tất cả tickers)
  - Formula: S_new = S_prev × (1 + r)
- [ ] Implement Jump Diffusion simulation:
  - Base diffusion + random jumps
  - Formula: S_new = S_prev × exp(base + jump)

### 4.4 Portfolio Value Calculation
- [ ] Implement function tính portfolio value:
  - FOR each ticker: ticker_value = shares[ticker] × price[ticker]
  - portfolio_value = Σ(ticker_value)
  - Store vào portfolio_paths[sim][t]

### 4.5 Main Simulation Loop
- [ ] Implement nested loop structure:
  - Outer loop: FOR sim = 1 TO N
  - Inner loop: FOR t = 1 TO total_steps
  - Call các functions: generate random, simulate price, calculate portfolio
- [ ] Implement progress tracking:
  - Emit progress updates (every 100 simulations)
  - Allow cancellation
- [ ] Optimize performance:
  - Vectorization nếu possible
  - Parallel processing nếu có nhiều cores

---

## Phase 5: Statistical Analysis & Output (Backend/Logic + UI)

### 5.1 Metrics Calculation (Per Simulation)
- [ ] Implement function tính metrics cho mỗi simulation:
  - Total Return: (end_value - initial) / initial
  - Annualized Return (CAGR): (end_value / initial)^(1/T) - 1
  - Annualized Volatility: std(daily_returns) × √252
  - Sharpe Ratio: (CAGR - Rf) / volatility
  - Sortino Ratio: (CAGR - Rf) / downside_volatility
  - Maximum Drawdown: max((peak - current) / peak)

### 5.2 Percentiles Calculation
- [ ] Implement function tính percentiles:
  - Sort tất cả metrics arrays
  - Calculate: p10, p25, p50, p75, p90
  - Apply cho: end_values, total_returns, CAGR, volatility, Sharpe, Sortino, max_dd

### 5.3 Probability Analysis
- [ ] Implement function tính probabilities:
  - P(Gain > 0%)
  - P(Beat Bank Rate)
  - P(Double Initial)
  - P(Lose > 25%)
  - P(Lose > 50%)

### 5.4 Time-Series Data Preparation
- [ ] Implement function tính yearly percentiles:
  - FOR year = 0 TO T
  - Extract values tại t = year × 252
  - Calculate percentiles cho từng year
  - Output: array of percentiles over time

### 5.5 Histogram Data Preparation
- [ ] Implement function tạo histogram data:
  - Filter 95% results (remove outliers)
  - Create bins (default: 20 bins)
  - Count frequency cho mỗi bin
  - Format labels (currency format)

---

## Phase 6: Results Visualization (UI)

### 6.1 Portfolio Balance Line Chart
- [ ] Tạo Line Chart component với:
  - X-axis: Years (0 to T)
  - Y-axis: Portfolio Value (VND)
  - 5 lines: 10th, 25th, 50th, 75th, 90th percentiles
- [ ] Styling:
  - Different colors/styles cho mỗi percentile
  - Legend với labels rõ ràng
  - Tooltips hiển thị giá trị khi hover
  - Format currency cho Y-axis

### 6.2 End Balance Histogram
- [ ] Tạo Histogram component với:
  - X-axis: Ending Portfolio Value (VND)
  - Y-axis: Frequency
  - Bars representing distribution
- [ ] Styling:
  - Bars có màu gradient
  - Title: "Portfolio End Balance Histogram (95% of results)"
  - Format currency cho X-axis

### 6.3 Performance Summary Panel
- [ ] Tạo section "Portfolio Allocation":
  - Table với columns: Ticker, Weight, Allocated, Shares, μ, σ
  - Row cuối cùng: Portfolio totals
- [ ] Tạo section "Correlation Matrix":
  - Table hiển thị correlation matrix
  - Color-coded cells (positive: green, negative: red)
- [ ] Tạo section "Simulation Parameters":
  - List các parameters đã sử dụng
- [ ] Tạo section "Return Metrics":
  - Table với rows: End Value, Total Return, CAGR, Volatility
  - Columns: 10th, 25th, 50th, 75th, 90th percentiles
- [ ] Tạo section "Risk-Adjusted Metrics":
  - Table với rows: Sharpe Ratio, Sortino Ratio, Maximum Drawdown
  - Columns: 10th, 25th, 50th, 75th, 90th percentiles
- [ ] Tạo section "Probability Analysis":
  - Display các probabilities dưới dạng % với icons
- [ ] Tạo section "Confidence Interval":
  - Highlight khoảng giá trị theo confidence interval đã chọn
  - Interpretation text
- [ ] Tạo section "Diversification Benefit":
  - So sánh weighted average volatility vs portfolio volatility
  - Hiển thị % reduction

### 6.4 Results Layout & Navigation
- [ ] Organize output sections theo thứ tự:
  1. Charts (Line Chart + Histogram) ở top
  2. Performance Summary ở dưới
- [ ] Implement responsive layout:
  - Desktop: 2 charts side-by-side
  - Mobile: stacked vertically
- [ ] Add export functionality:
  - Export results to PDF
  - Export data to CSV/Excel

---

## Phase 7: Error Handling & Validation

### 7.1 Input Validation
- [ ] Validate portfolio allocation:
  - At least 1 ticker
  - Weights sum to 100%
  - Valid ticker symbols
- [ ] Validate basic parameters:
  - Initial amount > 0
  - Period > 0 và reasonable
  - Number of simulations trong range hợp lý
- [ ] Validate model parameters:
  - Manual volatilities > 0
  - Risk-free rate reasonable

### 7.2 Data Fetching Error Handling
- [ ] Handle API errors:
  - Ticker không tồn tại
  - Network timeout
  - Server errors
- [ ] Handle insufficient data:
  - Không đủ historical data
  - Missing data points
  - Data quality issues

### 7.3 Calculation Error Handling
- [ ] Handle numerical issues:
  - Division by zero
  - Overflow/underflow
  - NaN values
- [ ] Handle matrix computation errors:
  - Non-positive definite correlation matrix
  - Singular matrix
  - Cholesky decomposition failures

### 7.4 User Feedback
- [ ] Implement loading indicators:
  - Progress bar khi fetch data
  - Progress bar khi run simulation
  - Estimated time remaining
- [ ] Implement error messages:
  - Clear, actionable error messages
  - Suggestions để fix errors
- [ ] Implement success notifications:
  - Confirmation khi simulation complete
  - Summary of results

---

## Phase 8: Testing & Optimization

### 8.1 Unit Testing
- [ ] Test portfolio allocation validation
- [ ] Test returns calculation
- [ ] Test correlation matrix calculation
- [ ] Test Cholesky decomposition
- [ ] Test simulation models (GBM, Historical, etc.)
- [ ] Test statistics calculations
- [ ] Test percentiles & probabilities

### 8.2 Integration Testing
- [ ] Test complete flow từ input đến output
- [ ] Test với different portfolio configurations
- [ ] Test với different models
- [ ] Test với edge cases (1 ticker, many tickers)

### 8.3 Performance Testing
- [ ] Measure execution time for different N simulations
- [ ] Identify performance bottlenecks
- [ ] Optimize slow functions
- [ ] Test memory usage

### 8.4 UI/UX Testing
- [ ] Test input interactions
- [ ] Test validation feedback
- [ ] Test charts rendering
- [ ] Test responsive layout
- [ ] Test cross-browser compatibility

---

## Phase 9: Documentation & Polish

### 9.1 Code Documentation
- [ ] Add comments cho các functions phức tạp
- [ ] Document các formulas sử dụng
- [ ] Document data structures
- [ ] Add JSDoc/TypeScript types

### 9.2 User Documentation
- [ ] Viết user guide:
  - How to input portfolio
  - How to interpret results
  - What each metric means
- [ ] Add tooltips trong UI:
  - Explain model options
  - Explain metrics (Sharpe, Sortino, etc.)
  - Explain confidence intervals

### 9.3 Final Polish
- [ ] Review UI styling và consistency
- [ ] Optimize chart appearance
- [ ] Add animations nếu appropriate
- [ ] Final testing trên các devices

---

## Dependencies & Requirements

### Technical Requirements
- Backend/Logic language: Rust/Python/JavaScript (tùy stack hiện tại)
- UI framework: Slint (theo file đã mở)
- Math libraries cần thiết:
  - Linear algebra (matrix operations)
  - Statistics (mean, std, correlation)
  - Random number generation
  - Numerical optimization (cho GARCH fitting)

### External APIs
- Historical price API (đã có sẵn trong hệ thống)

### Performance Requirements
- Simulation với 10,000 iterations phải complete trong < 30 seconds
- UI phải responsive (không freeze) khi run simulation
- Support portfolio size lên đến 20 tickers

---

## Priority & Estimation

### High Priority (MVP)
1. Phase 1: User Input Interface - 3-5 days
2. Phase 2: Data Fetching - 2-3 days
3. Phase 3: Model Parameters (chỉ GBM) - 2-3 days
4. Phase 4: Monte Carlo Engine (chỉ GBM Log-normal) - 3-4 days
5. Phase 5: Statistical Analysis - 2-3 days
6. Phase 6: Results Visualization (basic) - 3-4 days

**Total MVP: ~15-22 days**

### Medium Priority
- Phase 3-4: Thêm Historical Returns model - 2 days
- Phase 6: Polish visualization - 2 days
- Phase 7: Comprehensive error handling - 2 days

### Low Priority (Future Enhancement)
- Phase 3-4: Jump Diffusion model - 3 days
- Phase 3-4: GARCH model - 4 days
- Phase 6: Export functionality - 1 day
- Phase 9: Full documentation - 2 days

---

## Notes
- Sprint này focus vào multiple tickers portfolio, khác với single ticker version cũ
- Correlation matrix là key difference và cần implement carefully
- Cholesky decomposition quan trọng để generate correlated random numbers
- UI cần flexible để support dynamic portfolio (add/remove tickers)
- Performance optimization quan trọng vì simulation có thể chậm với nhiều tickers
