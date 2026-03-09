import SwiftUI

extension Color {
    static let erpPrimary = Color(red: 21/255, green: 101/255, blue: 192/255)       // #1565C0
    static let erpPrimaryLight = Color(red: 66/255, green: 165/255, blue: 245/255)  // #42A5F5
    static let erpPrimaryDark = Color(red: 13/255, green: 71/255, blue: 161/255)    // #0D47A1
    static let erpAccent = Color(red: 255/255, green: 143/255, blue: 0/255)         // #FF8F00
    static let erpSuccess = Color(red: 46/255, green: 125/255, blue: 50/255)        // #2E7D32
    static let erpWarning = Color(red: 245/255, green: 124/255, blue: 0/255)        // #F57C00
    static let erpError = Color(red: 211/255, green: 47/255, blue: 47/255)          // #D32F2F
    static let erpBackground = Color(uiColor: .systemGroupedBackground)
    static let erpCardBackground = Color(uiColor: .secondarySystemGroupedBackground)

    static func statusColor(for status: String) -> Color {
        switch status.lowercased() {
        case "draft", "pending", "new", "created":
            return .gray
        case "released", "open", "in_progress", "active", "confirmed":
            return .erpPrimary
        case "completed", "closed", "approved", "conforming", "received":
            return .erpSuccess
        case "cancelled", "rejected", "non_conforming":
            return .erpError
        case "partially_delivered", "partial", "partially_received":
            return .erpWarning
        default:
            return .secondary
        }
    }
}

// Allow .erpPrimary etc. in foregroundStyle() and other ShapeStyle contexts
extension ShapeStyle where Self == Color {
    static var erpPrimary: Color { Color.erpPrimary }
    static var erpPrimaryLight: Color { Color.erpPrimaryLight }
    static var erpPrimaryDark: Color { Color.erpPrimaryDark }
    static var erpAccent: Color { Color.erpAccent }
    static var erpSuccess: Color { Color.erpSuccess }
    static var erpWarning: Color { Color.erpWarning }
    static var erpError: Color { Color.erpError }
    static var erpBackground: Color { Color.erpBackground }
    static var erpCardBackground: Color { Color.erpCardBackground }
}
