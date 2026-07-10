export const API_CONFIG = {
  RUST_BACKEND: 'http://localhost:9090/api/v1/internal',
  PYTHON_SERVICE: 'http://localhost:9000/api/v1/internal',
  TIMEOUT: 10000,
};

export const fetchWithTimeout = async (url: string, options: RequestInit = {}, timeout: number = API_CONFIG.TIMEOUT) => {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  try {
    const response = await fetch(url, {
      ...options,
      signal: controller.signal,
    });
    clearTimeout(timeoutId);
    return response;
  } catch (error) {
    clearTimeout(timeoutId);
    throw error;
  }
};
