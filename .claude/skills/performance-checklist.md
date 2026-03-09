# 效能優化檢查清單

## 目錄
1. [iOS 效能優化](#ios-效能優化)
2. [Android 效能優化](#android-效能優化)
3. [通用記憶體優化](#通用記憶體優化)
4. [網路優化](#網路優化)
5. [效能檢測工具](#效能檢測工具)

## iOS 效能優化

### SwiftUI View 建構
- [ ] **避免在 body 中做複雜計算** - 將運算移至 ViewModel
- [ ] **拆分大型 View** - 超過 100 行應拆分為子 View
- [ ] **使用 EquatableView** - 避免不必要的重繪
- [ ] **善用 @ViewBuilder** - 條件式 View 建構

```swift
// Avoid - computation in body
var body: some View {
    let filtered = items.filter { $0.isActive }.sorted(by: { $0.date > $1.date })
    List(filtered) { item in ItemRow(item: item) }
}

// Prefer - computation in ViewModel
var body: some View {
    List(viewModel.filteredItems) { item in ItemRow(item: item) }
}
```

### SwiftUI 狀態管理效能
- [ ] **精確使用 @State vs @StateObject** - 避免不必要的 View 更新
- [ ] **使用 @ObservedObject 的子 View 拆分** - 限制更新範圍
- [ ] **善用 .task / .onAppear** - 正確時機載入資料

```swift
// Prefer - split sub-views to limit re-rendering
struct ParentView: View {
    @StateObject private var viewModel = ParentViewModel()

    var body: some View {
        VStack {
            HeaderView(title: viewModel.title)  // Only re-renders when title changes
            CounterView(count: viewModel.count)  // Only re-renders when count changes
        }
    }
}
```

### List/ScrollView 優化
- [ ] **使用 LazyVStack / LazyHStack** - 大量項目懶加載
- [ ] **避免在 List 中使用 .id()** - 防止整個 List 重建
- [ ] **設定適當的 prefetch** - 預載入資料

```swift
ScrollView {
    LazyVStack(spacing: 8) {
        ForEach(viewModel.items) { item in
            ItemRow(item: item)
                .id(item.id)  // Stable identity
        }
    }
}
```

### iOS 動畫優化
- [ ] **使用 withAnimation 限定範圍** - 只動畫需要的部分
- [ ] **避免在動畫中建立物件** - 預先定義動畫參數
- [ ] **使用 .drawingGroup()** - 複雜 View 合併為單一圖層

```swift
// Prefer - scoped animation
withAnimation(.easeOut(duration: 0.3)) {
    isExpanded.toggle()
}

// For complex views
ComplexView()
    .drawingGroup()  // Renders as a single bitmap
```

## Android 效能優化

### Jetpack Compose 建構
- [ ] **使用 remember 快取計算結果** - 避免每次 recomposition 重新計算
- [ ] **拆分 Composable** - 限制 recomposition 範圍
- [ ] **使用 derivedStateOf** - 衍生狀態避免不必要更新
- [ ] **避免在 Composable 中建立物件** - 使用 remember

```kotlin
// Avoid - recalculated every recomposition
@Composable
fun ItemList(items: List<Item>) {
    val sorted = items.sortedBy { it.date }  // Runs every recomposition
    LazyColumn { items(sorted) { ItemRow(it) } }
}

// Prefer - cached with remember
@Composable
fun ItemList(items: List<Item>) {
    val sorted = remember(items) { items.sortedBy { it.date } }
    LazyColumn { items(sorted) { ItemRow(it) } }
}
```

### Compose 狀態管理效能
- [ ] **使用 derivedStateOf** - 減少不必要的 recomposition
- [ ] **正確使用 key** - 穩定的 item identity
- [ ] **避免不必要的 recomposition** - 檢查穩定性

```kotlin
// derivedStateOf - only recomposes when result changes
val showButton by remember {
    derivedStateOf { listState.firstVisibleItemIndex > 0 }
}

// Stable key for LazyColumn
LazyColumn {
    items(items, key = { it.id }) { item ->
        ItemRow(item = item)
    }
}
```

### RecyclerView / LazyColumn 優化
- [ ] **使用 LazyColumn 而非 Column** - 大量項目懶加載
- [ ] **設定 contentType** - 不同類型的 item
- [ ] **使用 key 參數** - 穩定的 item identity

```kotlin
LazyColumn {
    items(
        items = items,
        key = { it.id },
        contentType = { it.type }  // Helps with view recycling
    ) { item ->
        ItemRow(item = item)
    }
}
```

### Android 動畫優化
- [ ] **使用 Animatable** - 可取消的動畫
- [ ] **使用 animateContentSize** - 簡單的大小動畫
- [ ] **避免在動畫中分配記憶體** - 預先定義動畫規格

```kotlin
// Animate content size changes
Box(modifier = Modifier.animateContentSize(
    animationSpec = spring(dampingRatio = Spring.DampingRatioMediumBouncy)
)) {
    if (expanded) ExpandedContent() else CollapsedContent()
}
```

## 通用記憶體優化

### 圖片處理
- [ ] **iOS: 使用 AsyncImage 或 SDWebImage** - 智能快取
- [ ] **Android: 使用 Coil/Glide** - 記憶體管理
- [ ] **指定圖片尺寸** - 降低解碼後記憶體
- [ ] **及時釋放圖片** - 避免記憶體洩漏

**iOS:**
```swift
AsyncImage(url: URL(string: imageURL)) { phase in
    switch phase {
    case .empty: ProgressView()
    case .success(let image):
        image.resizable()
            .aspectRatio(contentMode: .fill)
            .frame(width: 200, height: 200)
    case .failure: Image(systemName: "photo")
    @unknown default: EmptyView()
    }
}
```

**Android:**
```kotlin
AsyncImage(
    model = ImageRequest.Builder(LocalContext.current)
        .data(imageUrl)
        .size(200, 200)
        .crossfade(true)
        .build(),
    contentDescription = null,
    placeholder = painterResource(R.drawable.placeholder),
    error = painterResource(R.drawable.error),
)
```

### 資源釋放
- [ ] **iOS: 正確管理 Task 生命週期** - 取消不需要的 Task
- [ ] **Android: 正確管理 coroutine scope** - viewModelScope 自動取消
- [ ] **避免記憶體洩漏** - 檢查強引用循環

**iOS:**
```swift
struct ContentView: View {
    @StateObject private var viewModel = ContentViewModel()

    var body: some View {
        Text("Hello")
            .task {
                // Automatically cancelled when view disappears
                await viewModel.loadData()
            }
    }
}
```

**Android:**
```kotlin
class ContentViewModel : ViewModel() {
    init {
        // Automatically cancelled when ViewModel is cleared
        viewModelScope.launch {
            loadData()
        }
    }
}
```

## 網路優化

### 請求策略
- [ ] **請求合併** - 避免重複請求相同資料
- [ ] **取消機制** - 頁面離開時取消進行中的請求
- [ ] **快取策略** - 合理使用 HTTP 快取與本地快取

### 資料載入
- [ ] **分頁載入** - 大量資料使用 infinite scroll
- [ ] **預載入** - 適當預載下一頁
- [ ] **樂觀更新** - UI 先更新，失敗再回滾

**iOS 分頁載入:**
```swift
@MainActor
class ProductListViewModel: ObservableObject {
    @Published var products: [Product] = []
    @Published var isLoadingMore = false
    private var currentPage = 1
    private var hasMorePages = true

    func loadMore() async {
        guard !isLoadingMore, hasMorePages else { return }
        isLoadingMore = true
        do {
            let response = try await repository.getProducts(page: currentPage + 1)
            products.append(contentsOf: response.data)
            currentPage += 1
            hasMorePages = !response.data.isEmpty
        } catch { /* handle error */ }
        isLoadingMore = false
    }
}
```

**Android 分頁載入:**
```kotlin
class ProductListViewModel @Inject constructor(
    private val repository: ProductRepository
) : ViewModel() {
    val products = Pager(PagingConfig(pageSize = 20)) {
        ProductPagingSource(repository)
    }.flow.cachedIn(viewModelScope)
}

// In Composable
val products = viewModel.products.collectAsLazyPagingItems()
LazyColumn {
    items(products.itemCount) { index ->
        products[index]?.let { ProductRow(it) }
    }
}
```

## 效能檢測工具

### iOS - Instruments
```bash
# Profile with Instruments
xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator' build-for-testing
# Open Instruments -> Time Profiler, Allocations, Leaks
```

**關鍵指標**
- Frame rendering time < 16ms (60fps)
- Memory usage and leak detection
- CPU usage during scrolling

### Android - Android Studio Profiler
```bash
# Run with profiling
./gradlew :app:installDebug
# Open Android Studio -> View -> Tool Windows -> Profiler
```

**關鍵指標**
- Frame rendering time < 16ms (60fps)
- Compose recomposition count
- Memory allocation pattern

### 效能預算
| 指標 | 目標 | 警告閾值 |
|------|------|----------|
| 首次繪製 (FCP) | < 1.5s | 2.5s |
| 可互動時間 (TTI) | < 3s | 5s |
| 幀率 | 60fps | < 55fps |
| iOS App 大小 | < 30MB | 50MB |
| Android APK 大小 | < 15MB | 25MB |
| 記憶體使用 | < 150MB | 250MB |
