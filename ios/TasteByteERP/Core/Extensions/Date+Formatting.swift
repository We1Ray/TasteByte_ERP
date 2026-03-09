import Foundation

extension Date {
    func formatted(as style: DateFormatStyle) -> String {
        let formatter = DateFormatter()
        formatter.locale = Locale.current

        switch style {
        case .short:
            formatter.dateStyle = .short
            formatter.timeStyle = .none
        case .medium:
            formatter.dateStyle = .medium
            formatter.timeStyle = .none
        case .long:
            formatter.dateStyle = .long
            formatter.timeStyle = .none
        case .dateTime:
            formatter.dateStyle = .medium
            formatter.timeStyle = .short
        case .timeOnly:
            formatter.dateStyle = .none
            formatter.timeStyle = .short
        case .iso:
            formatter.dateFormat = "yyyy-MM-dd"
        case .dayMonth:
            formatter.dateFormat = "dd MMM"
        case .monthYear:
            formatter.dateFormat = "MMM yyyy"
        }

        return formatter.string(from: self)
    }

    func relativeDescription() -> String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .abbreviated
        return formatter.localizedString(for: self, relativeTo: Date())
    }
}

enum DateFormatStyle {
    case short
    case medium
    case long
    case dateTime
    case timeOnly
    case iso
    case dayMonth
    case monthYear
}

extension Double {
    var currencyFormatted: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = "USD"
        return formatter.string(from: NSNumber(value: self)) ?? "$\(self)"
    }

    var quantityFormatted: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 2
        return formatter.string(from: NSNumber(value: self)) ?? "\(self)"
    }
}
