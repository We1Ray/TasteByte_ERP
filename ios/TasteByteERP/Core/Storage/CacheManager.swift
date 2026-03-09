import Foundation

final class CacheManager {
    static let shared = CacheManager()

    private let fileManager = FileManager.default
    private let cacheDir: URL
    private let encoder = JSONEncoder()
    private let decoder = JSONDecoder()

    private init() {
        cacheDir = fileManager.urls(for: .cachesDirectory, in: .userDomainMask)[0]
            .appendingPathComponent("api_cache")
        try? fileManager.createDirectory(at: cacheDir, withIntermediateDirectories: true)
    }

    func save<T: Encodable>(_ data: T, forKey key: String, ttl: TimeInterval = 3600) {
        let wrapper = CacheEntry(data: data, expiresAt: Date().addingTimeInterval(ttl))
        guard let encoded = try? encoder.encode(wrapper) else { return }
        let fileURL = cacheDir.appendingPathComponent(key.sanitizedFileName)
        try? encoded.write(to: fileURL)
    }

    func get<T: Decodable>(forKey key: String) -> T? {
        let fileURL = cacheDir.appendingPathComponent(key.sanitizedFileName)
        guard let data = try? Data(contentsOf: fileURL) else { return nil }
        guard let wrapper = try? decoder.decode(CacheEntry<T>.self, from: data) else { return nil }
        guard wrapper.expiresAt > Date() else {
            try? fileManager.removeItem(at: fileURL)
            return nil
        }
        return wrapper.data
    }

    func clear() {
        try? fileManager.removeItem(at: cacheDir)
        try? fileManager.createDirectory(at: cacheDir, withIntermediateDirectories: true)
    }
}

private struct CacheEntry<T: Codable>: Codable {
    let data: T
    let expiresAt: Date
}

private extension String {
    var sanitizedFileName: String {
        let allowed = CharacterSet.alphanumerics.union(CharacterSet(charactersIn: "-_"))
        return self.unicodeScalars
            .map { allowed.contains($0) ? String($0) : "_" }
            .joined()
    }
}
