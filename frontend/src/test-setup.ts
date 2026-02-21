import '@testing-library/jest-dom';

// Mock @tauri-apps/api for tests
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));
