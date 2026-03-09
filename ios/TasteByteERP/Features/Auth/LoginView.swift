import SwiftUI

struct LoginView: View {
    @EnvironmentObject var authManager: AuthManager
    @StateObject private var viewModel = LoginViewModel()
    @FocusState private var focusedField: Field?

    private enum Field {
        case username, password
    }

    var body: some View {
        GeometryReader { geometry in
            ScrollView {
                VStack(spacing: 0) {
                    Spacer()
                        .frame(height: geometry.size.height * 0.12)

                    // Logo and Title
                    VStack(spacing: 12) {
                        Image(systemName: "building.2.fill")
                            .font(.system(size: 56))
                            .foregroundStyle(.erpPrimary)

                        Text("TasteByte ERP")
                            .font(.largeTitle)
                            .fontWeight(.bold)

                        Text("Enterprise Resource Planning")
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                    }
                    .padding(.bottom, 48)

                    // Login Form
                    VStack(spacing: 16) {
                        VStack(alignment: .leading, spacing: 6) {
                            Text("Username")
                                .font(.subheadline)
                                .fontWeight(.medium)
                                .foregroundStyle(.secondary)

                            TextField("Enter your username", text: $viewModel.username)
                                .textFieldStyle(.plain)
                                .padding(12)
                                .background(
                                    RoundedRectangle(cornerRadius: 10)
                                        .fill(Color(uiColor: .tertiarySystemFill))
                                )
                                .textContentType(.username)
                                .textInputAutocapitalization(.never)
                                .autocorrectionDisabled()
                                .focused($focusedField, equals: .username)
                                .submitLabel(.next)
                                .onSubmit { focusedField = .password }
                        }

                        VStack(alignment: .leading, spacing: 6) {
                            Text("Password")
                                .font(.subheadline)
                                .fontWeight(.medium)
                                .foregroundStyle(.secondary)

                            SecureField("Enter your password", text: $viewModel.password)
                                .textFieldStyle(.plain)
                                .padding(12)
                                .background(
                                    RoundedRectangle(cornerRadius: 10)
                                        .fill(Color(uiColor: .tertiarySystemFill))
                                )
                                .textContentType(.password)
                                .focused($focusedField, equals: .password)
                                .submitLabel(.go)
                                .onSubmit {
                                    Task { await viewModel.login(authManager: authManager) }
                                }
                        }

                        if let error = viewModel.errorMessage {
                            HStack(spacing: 6) {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .font(.caption)
                                Text(error)
                                    .font(.caption)
                            }
                            .foregroundStyle(.erpError)
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .padding(.top, 4)
                        }

                        Button {
                            focusedField = nil
                            Task { await viewModel.login(authManager: authManager) }
                        } label: {
                            Group {
                                if viewModel.isLoading {
                                    ProgressView()
                                        .tint(.white)
                                } else {
                                    Text("Sign In")
                                        .fontWeight(.semibold)
                                }
                            }
                            .frame(maxWidth: .infinity)
                            .frame(height: 20)
                        }
                        .buttonStyle(.borderedProminent)
                        .tint(.erpPrimary)
                        .controlSize(.large)
                        .disabled(!viewModel.isFormValid || viewModel.isLoading)
                        .padding(.top, 8)
                    }
                    .padding(.horizontal, 32)

                    Spacer()
                }
                .frame(minHeight: geometry.size.height)
            }
        }
        .background(Color.erpBackground)
        .onTapGesture { focusedField = nil }
    }
}
