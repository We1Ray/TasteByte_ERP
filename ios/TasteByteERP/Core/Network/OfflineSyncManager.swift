import Foundation
import Combine

final class OfflineSyncManager: ObservableObject {
    static let shared = OfflineSyncManager()

    @Published var pendingCount = 0

    private let fileManager = FileManager.default
    private let queueDir: URL
    private let encoder = JSONEncoder()
    private let decoder = JSONDecoder()
    private var cancellable: AnyCancellable?

    private init() {
        queueDir = fileManager.urls(for: .documentDirectory, in: .userDomainMask)[0]
            .appendingPathComponent("offline_queue")
        try? fileManager.createDirectory(at: queueDir, withIntermediateDirectories: true)
        pendingCount = loadQueue().count

        cancellable = NetworkMonitor.shared.$isConnected
            .removeDuplicates()
            .filter { $0 }
            .sink { [weak self] _ in
                Task { await self?.syncPendingOperations() }
            }
    }

    func enqueue(method: String, endpoint: String, body: Data?) {
        let op = PendingOperation(
            id: UUID().uuidString,
            method: method,
            endpoint: endpoint,
            body: body,
            createdAt: Date()
        )
        guard let data = try? encoder.encode(op) else { return }
        let fileURL = queueDir.appendingPathComponent("\(op.id).json")
        try? data.write(to: fileURL)
        DispatchQueue.main.async {
            self.pendingCount += 1
        }
    }

    @MainActor
    func syncPendingOperations() async {
        let operations = loadQueue()
        guard !operations.isEmpty else { return }

        for op in operations {
            do {
                try await executeOperation(op)
                removeOperation(op.id)
                pendingCount = max(0, pendingCount - 1)
            } catch {
                // Stop syncing on first failure to preserve order
                break
            }
        }
    }

    private func loadQueue() -> [PendingOperation] {
        guard let files = try? fileManager.contentsOfDirectory(at: queueDir, includingPropertiesForKeys: nil) else {
            return []
        }
        return files
            .filter { $0.pathExtension == "json" }
            .compactMap { url -> PendingOperation? in
                guard let data = try? Data(contentsOf: url) else { return nil }
                return try? decoder.decode(PendingOperation.self, from: data)
            }
            .sorted { $0.createdAt < $1.createdAt }
    }

    private func removeOperation(_ id: String) {
        let fileURL = queueDir.appendingPathComponent("\(id).json")
        try? fileManager.removeItem(at: fileURL)
    }

    private func executeOperation(_ op: PendingOperation) async throws {
        guard var components = URLComponents(string: APIEndpoints.baseURL + op.endpoint) else {
            throw APIError.invalidURL
        }
        guard let url = components.url else {
            throw APIError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = op.method
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue("application/json", forHTTPHeaderField: "Accept")

        if let token = KeychainHelper.shared.readString(forKey: "com.tastebyte.erp.accessToken") {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        request.httpBody = op.body

        let (_, response) = try await URLSession.shared.data(for: request)
        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            throw APIError.unknown(0)
        }
    }
}

private struct PendingOperation: Codable {
    let id: String
    let method: String
    let endpoint: String
    let body: Data?
    let createdAt: Date
}
