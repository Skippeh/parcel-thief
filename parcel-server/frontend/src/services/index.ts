export interface ApiResponse<T> {
  data: T | null;
  statusCode: number;
  error: string | null;
}

let authToken: string | null = null;

export function setAuthToken(token: string | null) {
  authToken = token;
}

export function getApiUrl(apiRoute: string): string {
  return `/frontend/api/${apiRoute}`;
}

export async function callApi<T>(
  route: string,
  method: string,
  requestData?: any
): Promise<ApiResponse<T>> {
  let apiResponse: ApiResponse<T>;

  try {
    const headers = {
      "Content-Type": "application/json",
    };

    // If we have an auth token, set Authorization header
    if (authToken != null) {
      headers["Authorization"] = `Bearer ${authToken}`;
    }

    const response = await fetch(getApiUrl(route), {
      method,
      body: requestData == null ? null : JSON.stringify(requestData),
      headers,
    });

    apiResponse = (await response.json()) as ApiResponse<T>;
  } catch (err) {
    console.error("Api request failed:", err);
    apiResponse = {
      data: null,
      statusCode: -1,
      error: "Could not send api request or read response",
    };
  }

  return apiResponse;
}
