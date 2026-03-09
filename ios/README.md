# TasteByte ERP - iOS App

Native iOS app built with Swift and SwiftUI for the TasteByte ERP system.

## Requirements

- Xcode 15.0+
- iOS 17.0+
- Swift 5.9+

## Setup

### Opening in Xcode

1. Open Xcode
2. File > New > Project > iOS > App
3. Set Product Name to "TasteByteERP"
4. Set Organization Identifier to "com.tastebyte"
5. Select Swift and SwiftUI
6. Create the project at `ios/`
7. Replace the generated source files with the files in `TasteByteERP/`

### Alternative: Manual Xcode Project Setup

1. Create a new Xcode project as above
2. Delete the default ContentView.swift and App file
3. Drag all files from `TasteByteERP/` into the Xcode project navigator
4. Ensure all files are added to the TasteByteERP target

### Backend Configuration

The app connects to the backend API at `http://localhost:8000/api/v1`.

For local development, add the following to your `Info.plist` to allow HTTP connections:

```xml
<key>NSAppTransportSecurity</key>
<dict>
    <key>NSAllowsLocalNetworking</key>
    <true/>
</dict>
```

## Architecture

- **MVVM Pattern**: Views observe ViewModels, ViewModels call APIClient
- **Core**: Network layer, Auth management, Models, Extensions
- **Features**: Feature modules (Auth, Dashboard, Materials, Sales, HR, Warehouse, Quality)
- **SharedViews**: Reusable UI components

## Features

| Module | Functionality |
|--------|--------------|
| Dashboard | KPI cards, quick actions, recent activity |
| Materials | Browse materials, view details, stock overview |
| Sales | Sales orders list, order details with line items |
| HR | Clock in/out, employee directory |
| Warehouse | Warehouse list, stock counting |
| Quality | Inspection lots, inspection form with results |

## Theme

- Primary Color: #1565C0 (Blue)
- Success: #2E7D32
- Warning: #F57C00
- Error: #D32F2F
